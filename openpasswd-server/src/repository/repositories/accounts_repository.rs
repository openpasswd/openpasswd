use crate::repository::models::account::{NewAccount, NewAccountGroup, NewAccountPassword};
use crate::repository::Repository;
use async_trait::async_trait;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

#[async_trait]
pub trait AccountsRepository {
    async fn accounts_groups_list(&self, user_id: i32) -> Vec<entity::account_groups::Model>;
    async fn accounts_groups_find_by_id(
        &self,
        id: i32,
        user_id: i32,
    ) -> Option<entity::account_groups::Model>;
    async fn accounts_groups_insert(
        &self,
        account_group: NewAccountGroup,
    ) -> Result<entity::account_groups::Model, ()>;
    async fn accounts_insert(&self, account: NewAccount) -> Result<entity::accounts::Model, ()>;
    async fn accounts_list(&self, user_id: i32) -> Vec<entity::accounts::Model>;
    async fn accounts_list_by_group_id(
        &self,
        user_id: i32,
        group_id: i32,
    ) -> Vec<entity::accounts::Model>;
    async fn accounts_get_with_passwords_by_account_id(
        &self,
        account_id: i32,
        user_id: i32,
    ) -> Option<(
        entity::accounts::Model,
        Vec<entity::account_passwords::Model>,
    )>;

    async fn account_passwords_insert(
        &self,
        account_password: NewAccountPassword,
    ) -> Result<i32, ()>;
    async fn accounts_passwords_list_account_id(
        &self,
        account_id: i32,
    ) -> Vec<entity::account_passwords::Model>;
}

#[async_trait]
impl AccountsRepository for Repository {
    async fn accounts_groups_list(&self, user_id: i32) -> Vec<entity::account_groups::Model> {
        entity::account_groups::Entity::find()
            .filter(entity::account_groups::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .unwrap()
    }

    async fn accounts_groups_find_by_id(
        &self,
        id: i32,
        user_id: i32,
    ) -> Option<entity::account_groups::Model> {
        entity::account_groups::Entity::find()
            .filter(
                entity::account_groups::Column::Id
                    .eq(id)
                    .and(entity::account_groups::Column::UserId.eq(user_id)),
            )
            .one(&self.db)
            .await
            .unwrap()
    }

    async fn accounts_groups_insert(
        &self,
        account_group: NewAccountGroup,
    ) -> Result<entity::account_groups::Model, ()> {
        let account_group = entity::account_groups::ActiveModel {
            user_id: Set(account_group.user_id),
            name: Set(account_group.name),
            ..Default::default()
        };
        let result = account_group.insert(&self.db).await.unwrap();
        // TODO rethink insert with last_insert_id
        // let result = entity::account_groups::Entity::insert(acccount_group)
        //     .exec(&self.db)
        //     .await
        //     .unwrap();

        Ok(result)
    }

    async fn accounts_insert(&self, account: NewAccount) -> Result<entity::accounts::Model, ()> {
        let account = entity::accounts::ActiveModel {
            name: Set(account.name),
            user_id: Set(account.user_id),
            level: Set(account.level),
            account_groups_id: Set(account.account_groups_id),
            ..Default::default()
        };
        let result = account.insert(&self.db).await.unwrap();
        // TODO rethink insert with last_insert_id
        // let result = entity::accounts::Entity::insert(account)
        //     .exec(&self.db)
        //     .await
        //     .unwrap();
        Ok(result)
    }

    async fn accounts_list(&self, user_id: i32) -> Vec<entity::accounts::Model> {
        entity::accounts::Entity::find()
            .filter(entity::accounts::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .unwrap()
    }

    async fn accounts_list_by_group_id(
        &self,
        user_id: i32,
        group_id: i32,
    ) -> Vec<entity::accounts::Model> {
        entity::accounts::Entity::find()
            .filter(
                entity::accounts::Column::UserId
                    .eq(user_id)
                    .and(entity::accounts::Column::AccountGroupsId.eq(group_id)),
            )
            .all(&self.db)
            .await
            .unwrap()
    }

    async fn accounts_get_with_passwords_by_account_id(
        &self,
        account_id: i32,
        user_id: i32,
    ) -> Option<(
        entity::accounts::Model,
        Vec<entity::account_passwords::Model>,
    )> {
        let mut result = match entity::accounts::Entity::find()
            .find_with_related(entity::account_passwords::Entity)
            .filter(
                entity::accounts::Column::Id
                    .eq(account_id)
                    .and(entity::accounts::Column::UserId.eq(user_id)),
            )
            .all(&self.db)
            .await
        {
            Ok(result) => result,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };

        let account = if result.len() > 0 {
            result.remove(0)
        } else {
            return None;
        };

        Some(account)
        // let connection = &self.db.get().unwrap();
        // // TODO: Join it better =D, I want to use sqlx

        // let mut result = match accounts_dsl::accounts
        //     .filter(
        //         accounts_dsl::id
        //             .eq(&account_id)
        //             .and(accounts_dsl::user_id.eq(user_id)),
        //     )
        //     .load::<Account>(connection)
        // {
        //     Ok(result) => result,
        //     Err(e) => panic!("{e}"),
        // };

        // let account = if result.len() > 0 {
        //     result.remove(0)
        // } else {
        //     return None;
        // };

        // let passwords = match account_passwords_dsl::account_passwords
        //     .filter(account_passwords_dsl::account_id.eq(account_id))
        //     .load::<AccountPassword>(connection)
        // {
        //     Ok(result) => result,
        //     Err(e) => panic!("{e}"),
        // };

        // Some(AccountWithPassword {
        //     id: account.id,
        //     user_id: account.user_id,
        //     account_groups_id: account.account_groups_id,
        //     level: account.level,
        //     name: account.name,
        //     passwords,
        // })
    }

    async fn account_passwords_insert(
        &self,
        account_password: NewAccountPassword,
    ) -> Result<i32, ()> {
        let account_password = entity::account_passwords::ActiveModel {
            account_id: Set(account_password.account_id),
            username: Set(account_password.username),
            password: Set(account_password.password),
            created_date: Set(account_password.created_date),
            ..Default::default()
        };
        let result = entity::account_passwords::Entity::insert(account_password)
            .exec(&self.db)
            .await
            .unwrap();
        Ok(result.last_insert_id)
    }

    async fn accounts_passwords_list_account_id(
        &self,
        account_id: i32,
    ) -> Vec<entity::account_passwords::Model> {
        entity::account_passwords::Entity::find()
            .filter(entity::account_passwords::Column::AccountId.eq(account_id))
            .all(&self.db)
            .await
            .unwrap()
    }
}
