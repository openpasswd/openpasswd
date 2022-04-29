use std::collections::VecDeque;

use crate::repository::models::account::{
    Account, AccountGroup, AccountPassword, NewAccount, NewAccountGroup, NewAccountPassword,
};
use crate::repository::schema::account_groups;
use crate::repository::schema::account_groups::dsl as account_groups_dsl;
use crate::repository::schema::account_passwords;
use crate::repository::schema::account_passwords::dsl as account_passwords_dsl;
use crate::repository::schema::accounts;
use crate::repository::schema::accounts::dsl as accounts_dsl;
use crate::repository::Repository;
use diesel::prelude::*;

pub trait AccountsRepository {
    fn accounts_groups_list(&self, user_id: i32) -> Vec<AccountGroup>;
    fn accounts_groups_find_by_id_and_user_id(&self, id: i32, user_id: i32)
        -> Option<AccountGroup>;
    fn accounts_groups_insert(&self, account_group: NewAccountGroup) -> Result<AccountGroup, ()>;

    fn accounts_insert(&self, account: NewAccount) -> Result<Account, ()>;
    fn accounts_list_by_user_id(&self, user_id: i32) -> Vec<Account>;
    fn accounts_list_by_user_id_and_group_id(&self, user_id: i32, group_id: i32) -> Vec<Account>;

    fn account_passwords_insert(
        &self,
        account_password: NewAccountPassword,
    ) -> Result<AccountPassword, ()>;
    fn accounts_passwords_list_account_id(&self, account_id: i32) -> Vec<AccountPassword>;
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

    fn accounts_groups_find_by_id_and_user_id(
        &self,
        id: i32,
        user_id: i32,
    ) -> Option<AccountGroup> {
        let connection = &self.pool.get().unwrap();
        let mut result = match account_groups_dsl::account_groups
            .filter(
                account_groups_dsl::id
                    .eq(&id)
                    .and(account_groups_dsl::user_id.eq(user_id)),
            )
            .load::<AccountGroup>(connection)
        {
            Ok(result) => VecDeque::from(result),
            Err(e) => panic!("{e}"),
        };

        result.pop_front()
    }

    fn accounts_groups_insert(&self, account_group: NewAccountGroup) -> Result<AccountGroup, ()> {
        let connection = &self.pool.get().unwrap();
        let account_group = match diesel::insert_into(account_groups::table)
            .values(account_group)
            .get_result(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        };

        Ok(account_group)
    }

    fn accounts_insert(&self, account: NewAccount) -> Result<Account, ()> {
        // Todo validate group_id before
        if self
            .accounts_groups_find_by_id_and_user_id(account.account_groups_id, account.user_id)
            .is_none()
        {
            return Err(());
        }

        let connection = &self.pool.get().unwrap();
        let account = match diesel::insert_into(accounts::table)
            .values(account)
            .get_result(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        };

        Ok(account)
    }

    fn accounts_list_by_user_id(&self, user_id: i32) -> Vec<Account> {
        let connection = &self.pool.get().unwrap();

        match accounts_dsl::accounts
            .filter(accounts_dsl::user_id.eq(user_id))
            .load::<Account>(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        }
    }

    fn accounts_list_by_user_id_and_group_id(&self, user_id: i32, group_id: i32) -> Vec<Account> {
        let connection = &self.pool.get().unwrap();
        match accounts_dsl::accounts
            .filter(
                accounts_dsl::user_id
                    .eq(user_id)
                    .and(accounts_dsl::account_groups_id.eq(group_id)),
            )
            .load::<Account>(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        }
    }

    fn account_passwords_insert(
        &self,
        account_password: NewAccountPassword,
    ) -> Result<AccountPassword, ()> {
        let connection = &self.pool.get().unwrap();
        let account = match diesel::insert_into(account_passwords::table)
            .values(account_password)
            .get_result(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        };

        Ok(account)
    }

    fn accounts_passwords_list_account_id(&self, account_id: i32) -> Vec<AccountPassword> {
        let connection = &self.pool.get().unwrap();
        match account_passwords_dsl::account_passwords
            .filter(account_passwords_dsl::account_id.eq(account_id))
            .load::<AccountPassword>(connection)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        }
    }
}
