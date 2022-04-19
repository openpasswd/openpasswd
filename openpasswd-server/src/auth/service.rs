use super::dto::auth_error::{AuthError, AuthResult};
use super::dto::claims::Claims;
use crate::orm::device::Device;
use crate::orm::schema::devices::dsl as devices_dsl;
use crate::orm::schema::users::dsl as users_dsl;
use crate::{
    orm::{
        schema::users,
        user::{NewUser, User},
    },
    DynPgConnection,
};
use diesel::prelude::*;
use log::warn;
use openpasswd_model::auth::{LoginRequest, UserRegister, UserView};

pub struct AuthService {
    connection: DynPgConnection,
}

impl AuthService {
    pub fn new(connection: DynPgConnection) -> AuthService {
        AuthService { connection }
    }

    fn find_user_by_email(&self, email: &str, connection: &PgConnection) -> Option<User> {
        let mut result = match users_dsl::users
            .filter(users_dsl::email.eq(&email))
            .load::<User>(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        };

        if result.len() > 0 {
            Some(result.remove(0))
        } else {
            None
        }
    }

    fn verify(&self, login_password: &str, user: &User, connection: &PgConnection) -> AuthResult {
        if pwhash::sha512_crypt::verify(login_password, &user.password) == false {
            diesel::update(users_dsl::users)
                .filter(users_dsl::id.eq(user.id))
                .set((
                    users_dsl::fail_attempts.eq(user.fail_attempts + 1),
                    users_dsl::last_attempt.eq(diesel::dsl::now),
                ))
                .execute(connection)
                .unwrap();

            Err(AuthError::InvalidCredentials)
        } else {
            Ok(())
        }
    }

    fn find_device_name(
        &self,
        login: &LoginRequest,
        user: &User,
        connection: &PgConnection,
    ) -> Option<String> {
        if let Some(device_name) = login.device_name.as_ref() {
            match devices_dsl::devices
                .filter(
                    devices_dsl::user_id
                        .eq(&user.id)
                        .and(devices_dsl::name.eq(device_name)),
                )
                .load::<Device>(connection)
            {
                Ok(result) => {
                    if let Some(next) = result.first() {
                        Some(next.name.clone())
                    } else {
                        None
                    }
                }
                Err(e) => panic!("{e}"),
            }
        } else {
            None
        }
    }

    fn update_last_login(&self, user: &User, connection: &PgConnection) {
        let last_login_time: std::time::SystemTime = chrono::Utc::now().into();

        diesel::update(users_dsl::users)
            .filter(users_dsl::id.eq(user.id))
            // .set(users_dsl::last_login.eq(diesel::dsl::now))
            .set(users_dsl::last_login.eq(last_login_time))
            .execute(connection)
            .unwrap();
    }

    fn sign_token(&self, user: &User, device_name: Option<String>) -> AuthResult<String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(60))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.email.to_string(),
            device: device_name,
            exp: expiration as usize,
        };

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|e| AuthError::JwtEncode(e.to_string()))?;

        Ok(token)
    }

    pub fn login(self, login: &LoginRequest) -> AuthResult<String> {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let user = match self.find_user_by_email(&login.email, &*conn_guard) {
            Some(user) => user,
            None => return Err(AuthError::InvalidCredentials),
        };

        if false {
            return Err(AuthError::InvalidCredentials);
        }

        self.verify(&login.password, &user, &*conn_guard)?;

        let device_name = self.find_device_name(&login, &user, &*conn_guard);

        self.update_last_login(&user, &*&conn_guard);

        let token = self.sign_token(&user, device_name)?;
        Ok(token)
    }

    pub fn register(self, user: &UserRegister) -> Result<(), AuthError> {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let user = match self.find_user_by_email(&user.email, &*conn_guard) {
            Some(user) => user,
            None => return Err(AuthError::EmailAlreadyTaken),
        };

        let password = pwhash::sha512_crypt::hash(&user.password).unwrap();

        let new_user = NewUser {
            name: &user.name,
            email: &user.email,
            password: &password,
        };

        if let Err(e) = diesel::insert_into(users::table)
            .values(new_user)
            .execute(&*conn_guard)
        {
            panic!("{e}");
        }
        Ok(())
    }

    pub fn get_me(self, email: &str) -> Result<UserView, AuthError> {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let user = match self.find_user_by_email(&email, &*conn_guard) {
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
