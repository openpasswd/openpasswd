use crate::repository::models::user::NewUser;
use crate::repository::models::user_password_recovery::NewUserPasswordRecovery;
use crate::repository::Repository;
use async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[async_trait]
pub trait UsersRepository {
    async fn users_find_by_email(&self, email: &str) -> Option<entity::users::Model>;
    async fn users_find_by_id(&self, id: i32) -> Option<entity::users::Model>;
    async fn users_update_last_login(&self, user_id: i32);
    async fn users_update_fail_attempts(&self, user_id: i32, fail_attempts: i16);
    async fn users_update_password(&self, user_id: i32, password: String);
    async fn users_insert(&self, user: NewUser);
    async fn users_password_recovery_insert(&self, password_recovery: NewUserPasswordRecovery);
    async fn users_password_recovery_find_by_token(
        &self,
        token: &str,
    ) -> Option<entity::user_password_recovery::Model>;
    async fn users_password_recovery_invalide(&self, token: String);
}

#[async_trait]
impl UsersRepository for Repository {
    async fn users_find_by_email(&self, email: &str) -> Option<entity::users::Model> {
        entity::users::Entity::find()
            .filter(entity::users::Column::Email.eq(email))
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn users_find_by_id(&self, id: i32) -> Option<entity::users::Model> {
        entity::users::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn users_update_last_login(&self, user_id: i32) {
        let user = entity::users::ActiveModel {
            id: Set(user_id),
            last_login: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        entity::users::Entity::update(user)
            .exec(&self.db)
            .await
            .unwrap();
    }

    async fn users_update_fail_attempts(&self, user_id: i32, fail_attempts: i16) {
        let user = entity::users::ActiveModel {
            id: Set(user_id),
            fail_attempts: Set(fail_attempts),
            last_login: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        entity::users::Entity::update(user)
            .exec(&self.db)
            .await
            .unwrap();
    }

    async fn users_update_password(&self, user_id: i32, password: String) {
        let user = entity::users::ActiveModel {
            id: Set(user_id),
            password: Set(password),
            ..Default::default()
        };

        entity::users::Entity::update(user)
            .exec(&self.db)
            .await
            .unwrap();
    }

    async fn users_insert(&self, new_user: NewUser) {
        let user = entity::users::ActiveModel {
            name: Set(new_user.name),
            email: Set(new_user.email),
            password: Set(new_user.password),
            master_key: Set(new_user.master_key),
            ..Default::default()
        };
        entity::users::Entity::insert(user)
            .exec(&self.db)
            .await
            .unwrap();
    }

    async fn users_password_recovery_insert(&self, password_recovery: NewUserPasswordRecovery) {
        let user = entity::user_password_recovery::ActiveModel {
            user_id: Set(password_recovery.user_id),
            token: Set(password_recovery.token),
            issued_at: Set(password_recovery.issued_at),
            valid: Set(password_recovery.valid),
            ..Default::default()
        };
        entity::user_password_recovery::Entity::insert(user)
            .exec(&self.db)
            .await
            .unwrap();
    }

    async fn users_password_recovery_find_by_token(
        &self,
        token: &str,
    ) -> Option<entity::user_password_recovery::Model> {
        entity::user_password_recovery::Entity::find()
            .filter(entity::user_password_recovery::Column::Token.eq(token))
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn users_password_recovery_invalide(&self, token: String) {
        let user = entity::user_password_recovery::ActiveModel {
            token: Set(token),
            valid: Set(false),
            ..Default::default()
        };

        entity::user_password_recovery::Entity::update(user)
            .exec(&self.db)
            .await
            .unwrap();
    }
}
