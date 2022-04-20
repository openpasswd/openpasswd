use crate::repository::models::user::{NewUser, User};
use crate::repository::schema::users;
use crate::repository::schema::users::dsl as users_dsl;
use crate::repository::Repository;
use diesel::prelude::*;

pub trait UsersRepository {
    fn users_find_by_email(&self, email: &str) -> Option<User>;
    fn users_find_by_id(&self, id: i32) -> Option<User>;
    fn users_update_last_login(&self, user_id: i32);
    fn users_update_fail_attempts(&self, user_id: i32, fail_attempts: i16);
    fn users_insert(&self, user: NewUser);
}

impl UsersRepository for Repository {
    fn users_find_by_email(&self, email: &str) -> Option<User> {
        let connection = &self.pool.get().unwrap();
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

    fn users_find_by_id(&self, id: i32) -> Option<User> {
        let connection = &self.pool.get().unwrap();
        let mut result = match users_dsl::users
            .filter(users_dsl::id.eq(&id))
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

    fn users_update_last_login(&self, user_id: i32) {
        let connection = &self.pool.get().unwrap();
        let last_login_time: std::time::SystemTime = chrono::Utc::now().into();
        diesel::update(users_dsl::users)
            .filter(users_dsl::id.eq(user_id))
            // .set(users_dsl::last_login.eq(diesel::dsl::now))
            .set(users_dsl::last_login.eq(last_login_time))
            .execute(connection)
            .unwrap();
    }

    fn users_update_fail_attempts(&self, user_id: i32, fail_attempts: i16) {
        let connection = &self.pool.get().unwrap();
        diesel::update(users_dsl::users)
            .filter(users_dsl::id.eq(user_id))
            .set((
                users_dsl::fail_attempts.eq(fail_attempts),
                users_dsl::last_attempt.eq(diesel::dsl::now),
            ))
            .execute(connection)
            .unwrap();
    }

    fn users_insert(&self, new_user: NewUser) {
        let connection = &self.pool.get().unwrap();
        if let Err(e) = diesel::insert_into(users::table)
            .values(new_user)
            .execute(connection)
        {
            panic!("{e}");
        }
    }
}
