use super::dto::auth_error::{AuthError, AuthResult};
use super::dto::claims::Claims;
use crate::core::cache::Cache;
use crate::core::mail_service::{EmailAddress, MailService, MessageBody};
use crate::repository::models::user::NewUser;
use crate::repository::models::user_password_recovery::NewUserPasswordRecovery;
use crate::repository::repositories::devices_repository::DevicesRepository;
use crate::repository::repositories::users_repository::UsersRepository;
use chrono::{TimeZone, Utc};
use entity::users::Model as User;
use openpasswd_model::auth::{
    AccessToken, LoginRequest, PasswordRecoveryFinish, PasswordRecoveryStart, UserRegister,
    UserView,
};
use rand::distributions::Alphanumeric;
use rand::Rng;
use sha2::{Digest, Sha256};

pub struct AuthService<T>
where
    T: UsersRepository + DevicesRepository,
{
    repository: T,
    cache: Cache,
}

impl<T> AuthService<T>
where
    T: UsersRepository + DevicesRepository,
{
    pub fn new(repository: T, cache: Cache) -> AuthService<T> {
        AuthService { repository, cache }
    }

    fn verify_password(&self, hash_password: &str, password: &str) -> bool {
        argon2::verify_encoded(&hash_password, password.as_bytes()).unwrap()
    }

    async fn verify_user_password(&self, login_password: &str, user: &User) -> AuthResult {
        if self.verify_password(&user.password, login_password) {
            Ok(())
        } else {
            self.repository
                .users_update_fail_attempts(user.id, user.fail_attempts + 1)
                .await;
            Err(AuthError::InvalidCredentials)
        }
    }

    async fn find_device_name(&self, login: &LoginRequest, user: &User) -> Option<String> {
        if let Some(device_name) = login.device_name.as_ref() {
            self.repository
                .devices_find_device_name(user.id, device_name)
                .await
        } else {
            None
        }
    }

    fn sign_token(
        &self,
        user: &User,
        device_name: Option<String>,
        exp: i64,
    ) -> AuthResult<(String, String)> {
        let claims = Claims {
            jti: uuid::Uuid::new_v4().to_string(),
            sub: user.id,
            device: device_name,
            exp,
        };

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AuthError::JwtEncode(e.to_string()))?;

        Ok((token, claims.jti))
    }

    pub async fn login(self, login: &LoginRequest) -> AuthResult<AccessToken> {
        let user = match self.repository.users_find_by_email(&login.email).await {
            Some(user) => user,
            None => return Err(AuthError::InvalidCredentials),
        };

        // TODO: count wrong passwords

        self.verify_user_password(&login.password, &user).await?;

        let device_name = self.find_device_name(&login, &user).await;

        self.repository.users_update_last_login(user.id).await;

        let expire_at = chrono::Duration::minutes(60);
        let expire = chrono::Utc::now()
            .checked_add_signed(expire_at)
            .expect("valid timestamp")
            .timestamp();

        let (signed_token, jti) = self.sign_token(&user, device_name, expire)?;

        let key = format!("signed_token:{}:{}", user.id, jti);
        self.cache
            .set_and_expire(&key, 1, expire_at.num_seconds() as usize)
            .await;

        let token = AccessToken {
            access_token: signed_token,
            token_type: String::from("Bearer"),
            refresh_token: None,
        };

        Ok(token)
    }

    fn generate_string_vec_u8(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (&mut rng).sample_iter(Alphanumeric).take(size).collect()
    }

    pub fn hash_password(password: String) -> String {
        let salt = Self::generate_string_vec_u8(12);
        let config = argon2::Config::default();

        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

    pub async fn logout(self, claims: Claims) -> AuthResult {
        let key = format!("signed_token:{}:{}", claims.sub, claims.jti);
        // let expire = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(claims.exp, 0), Utc);
        // let expiretime = self.cache.get_expiretime(&key).await;
        // println!("Token expire: {}", expire.timestamp());
        // println!("Redis key expire: {expiretime}");

        self.cache.set_keepttl(&key, 0).await;

        Ok(())
    }

    pub async fn register(self, user: UserRegister) -> Result<(), AuthError> {
        if self
            .repository
            .users_find_by_email(&user.email)
            .await
            .is_some()
        {
            return Err(AuthError::EmailAlreadyTaken);
        }

        let UserRegister {
            name,
            email,
            password,
        } = user;

        let password = Self::hash_password(password);

        let id = uuid::Uuid::new_v4();
        let master_key = id.simple().to_string();

        let new_user = NewUser {
            name,
            email,
            password,
            master_key: Some(master_key),
        };

        self.repository.users_insert(new_user).await;
        Ok(())
    }

    pub async fn get_me(self, id: i32) -> Result<UserView, AuthError> {
        let user = match self.repository.users_find_by_id(id).await {
            Some(user) => user,
            None => return Err(AuthError::WrongCredentials),
        };

        let last_login_time = if let Some(last_login_time) = user.last_login {
            let datetime = Utc.from_utc_datetime(&last_login_time);
            Some(datetime.to_rfc3339())
        } else {
            None
        };

        Ok(UserView {
            email: user.email.to_owned(),
            name: user.name.to_owned(),
            last_login: last_login_time,
        })
    }

    pub async fn password_recovery_start(
        self,
        pass_recovery: PasswordRecoveryStart,
    ) -> Result<(), AuthError> {
        let user = match self
            .repository
            .users_find_by_email(&pass_recovery.email)
            .await
        {
            Some(user) => user,
            None => {
                log::warn!("User not found");
                return Ok(());
            }
        };

        let token = Self::generate_string_vec_u8(64);

        let hash_token = self.hash(&token);

        let user_password_recovery = NewUserPasswordRecovery {
            token: hash_token,
            user_id: user.id,
            issued_at: chrono::Utc::now().naive_utc(),
            valid: true,
        };

        self.repository
            .users_password_recovery_insert(user_password_recovery)
            .await;

        MailService::send_email(
            EmailAddress::new(Some("OpenPasswd"), "openpasswd@gmail.com"),
            EmailAddress::new(Some(&user.name), &user.email),
            String::from("Password recovery"),
            MessageBody::Text(format!(
                "Password recovery: {}",
                String::from_utf8(token).unwrap()
            )),
        )
        .await
        .unwrap();

        Ok(())
    }

    fn hash(&self, data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash_token = hasher.finalize();
        format!("{:x}", hash_token)
    }

    pub async fn password_recovery_finish(
        self,
        pass_recovery: PasswordRecoveryFinish,
    ) -> Result<(), AuthError> {
        let token = self.hash(&pass_recovery.token);

        let user_password_recovery = match self
            .repository
            .users_password_recovery_find_by_token(&token)
            .await
        {
            Some(user_password_recovery) => user_password_recovery,
            None => {
                log::warn!("User not found");
                return Ok(());
            }
        };

        if user_password_recovery.valid
            && user_password_recovery.issued_at + chrono::Duration::minutes(5)
                > chrono::Utc::now().naive_utc()
        {
            let password = Self::hash_password(pass_recovery.password);
            self.repository
                .users_password_recovery_invalide(token)
                .await;
            self.repository
                .users_update_password(user_password_recovery.user_id, password)
                .await;
        } else {
            log::warn!("Invalid password recovery token");
        }
        Ok(())
    }
}
