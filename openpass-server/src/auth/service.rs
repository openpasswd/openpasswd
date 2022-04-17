use crate::orm::schema::users::dsl;
use crate::{
    dto::UserRegister,
    orm::{
        schema::users,
        user::{NewUser, User},
    },
    DynPgConnection,
};
use diesel::prelude::*;
use log::{info, warn};

pub fn login() {}

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
    info!(
        "pwhash::sha512_crypt::verify: {}",
        pwhash::sha512_crypt::verify(&user.password, &password)
    );
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
