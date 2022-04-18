use crate::orm::schema::users::dsl;
use crate::{
    orm::{
        schema::users,
        user::{NewUser, User},
    },
    DynPgConnection,
};
use diesel::prelude::*;
use log::{info, warn};
use openpass_model::auth::{LoginRequest, UserRegister};

use super::dto::Claims;

pub fn login(login: &LoginRequest, connection: DynPgConnection) -> Result<String, String> {
    let conn_guard = match connection.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!("Lock is poisoned");
            poisoned.into_inner()
        }
    };

    let result = match dsl::users
        .filter(dsl::email.eq(&login.email))
        .load::<User>(&*conn_guard)
    {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    if result.len() == 0 {
        return Err(String::from("Email or password is incorrect"));
    }

    let user = result.first().unwrap();

    // TODO if users fail_attempts >= 5 and last_attempt < 10 min don't even try
    if false {
        return Err(String::from("Email or password is incorrect"));
    }

    if pwhash::sha512_crypt::verify(&login.password, &user.password) == false {
        diesel::update(dsl::users)
            .filter(dsl::id.eq(user.id))
            .set((
                dsl::fail_attempts.eq(user.fail_attempts + 1),
                dsl::last_attempt.eq(diesel::dsl::now),
            ))
            .execute(&*conn_guard)
            .unwrap();

        return Err(String::from("Email or password is incorrect"));
    }

    // TODO clean up fail_attempts after 24h+ last_attemps

    diesel::update(dsl::users)
        .filter(dsl::id.eq(user.id))
        .set(dsl::last_login.eq(diesel::dsl::now))
        .execute(&*conn_guard)
        .unwrap();

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.email.to_string(),
        device: "ACME".to_owned(),
        exp: expiration as usize,
    };

    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| format!("{e}"))?;

    Ok(token)
}

pub fn register(user: &UserRegister, connection: DynPgConnection) -> Result<(), String> {
    let conn_guard = match connection.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!("Lock is poisoned");
            poisoned.into_inner()
        }
    };

    let result = match dsl::users
        .filter(dsl::email.eq(&user.email))
        .load::<User>(&*conn_guard)
    {
        Ok(result) => result,
        Err(e) => panic!("{e}"),
    };

    if result.len() > 0 {
        return Err(String::from("Email already in use"));
    }

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
