use crate::repository::models::account_group::{AccountGroup, NewAccount, NewAccountGroup};
use crate::repository::schema::account_groups;
use crate::repository::schema::account_groups::dsl as account_groups_dsl;
use crate::repository::schema::accounts;
use crate::repository::Repository;
// use crate::orm::schema::accounts::dsl as accounts_dsl;
use diesel::prelude::*;

pub trait AccountsRepository {
    fn accounts_groups_list(&self, user_id: i32) -> Vec<AccountGroup>;
    fn accounts_insert(&self, account: NewAccount);
    fn accounts_groups_insert(&self, account_group: NewAccountGroup);
}

impl AccountsRepository for Repository {
    fn accounts_groups_list(&self, user_id: i32) -> Vec<AccountGroup> {
        let connection = &self.pool.get().unwrap();
        match account_groups_dsl::account_groups
            .filter(account_groups_dsl::user_id.eq(user_id))
            .load::<AccountGroup>(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        }
    }

    fn accounts_insert(&self, account: NewAccount) {
        let connection = &self.pool.get().unwrap();
        if let Err(e) = diesel::insert_into(accounts::table)
            .values(account)
            .execute(connection)
        {
            panic!("{e}");
        }
    }

    fn accounts_groups_insert(&self, account_group: NewAccountGroup) {
        let connection = &self.pool.get().unwrap();
        if let Err(e) = diesel::insert_into(account_groups::table)
            .values(account_group)
            .execute(connection)
        {
            panic!("{e}");
        }
    }
}
