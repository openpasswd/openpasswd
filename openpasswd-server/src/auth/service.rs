use super::dto::auth_error::{AuthError, AuthResult};
use super::dto::claims::Claims;
use crate::repository::models::user::{NewUser, User};
use crate::repository::repositories::devices_repository::DevicesRepository;
use crate::repository::repositories::users_repository::UsersRepository;
use openpasswd_model::auth::{LoginRequest, UserRegister, UserView};

pub struct AuthService<T>
where
    T: UsersRepository + DevicesRepository,
{
    repository: T,
}

impl<T> AuthService<T>
where
    T: UsersRepository + DevicesRepository,
{
    pub fn new(repository: T) -> AuthService<T> {
        AuthService { repository }
    }

    fn verify(&self, login_password: &str, user: &User) -> AuthResult {
        if pwhash::sha512_crypt::verify(login_password, &user.password) == false {
            self.repository
                .users_update_fail_attempts(user.id, user.fail_attempts + 1);
            Err(AuthError::InvalidCredentials)
        } else {
            Ok(())
        }
    }

    fn find_device_name(&self, login: &LoginRequest, user: &User) -> Option<String> {
        if let Some(device_name) = login.device_name.as_ref() {
            self.repository
                .devices_find_device_name(user.id, device_name)
        } else {
            None
        }
    }

    fn sign_token(&self, user: &User, device_name: Option<String>) -> AuthResult<String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id,
            device: device_name,
            exp: expiration as usize,
        };

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| AuthError::JwtEncode(e.to_string()))?;

        Ok(token)
    }

    pub fn login(self, login: &LoginRequest) -> AuthResult<String> {
        let user = match self.repository.users_find_by_email(&login.email) {
            Some(user) => user,
            None => return Err(AuthError::InvalidCredentials),
        };

        if false {
            return Err(AuthError::InvalidCredentials);
        }

        self.verify(&login.password, &user)?;

        let device_name = self.find_device_name(&login, &user);

        self.repository.users_update_last_login(user.id);

        let token = self.sign_token(&user, device_name)?;
        Ok(token)
    }

    pub fn register(self, user: &UserRegister) -> Result<(), AuthError> {
        if self.repository.users_find_by_email(&user.email).is_some() {
            return Err(AuthError::EmailAlreadyTaken);
        }

        let password = pwhash::sha512_crypt::hash(&user.password).unwrap();

        let new_user = NewUser {
            name: &user.name,
            email: &user.email,
            password: &password,
        };

        self.repository.users_insert(new_user);
        Ok(())
    }

    pub fn get_me(self, id: i32) -> Result<UserView, AuthError> {
        let user = match self.repository.users_find_by_id(id) {
            Some(user) => user,
            None => return Err(AuthError::WrongCredentials),
        };

        let last_login_time = if let Some(last_login_time) = user.last_login {
            let datetime: chrono::DateTime<chrono::Utc> = last_login_time.into();
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
}
