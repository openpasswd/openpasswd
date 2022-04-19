use crate::orm::account_group::{AccountGroup, NewAccount, NewAccountGroup};
use crate::orm::schema::account_groups;
use crate::orm::schema::account_groups::dsl as account_groups_dsl;
use crate::orm::schema::accounts;
// use crate::orm::schema::accounts::dsl as accounts_dsl;
use crate::DynPgConnection;
use diesel::prelude::*;
use log::warn;
use openpasswd_model::accounts::{AccountGroupRegister, AccountGroupView, AccountRegister};
use openpasswd_model::List;

use super::dto::accounts_error::AccountResult;

pub struct AccountService {
    connection: DynPgConnection,
}

impl AccountService {
    pub fn new(connection: DynPgConnection) -> AccountService {
        AccountService { connection }
    }

    pub fn register_group(self, account_group: &AccountGroupRegister, id: i32) -> AccountResult {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let account_group = NewAccountGroup {
            name: account_group.name.as_ref(),
            user_id: id,
        };

        if let Err(e) = diesel::insert_into(account_groups::table)
            .values(account_group)
            .execute(&*conn_guard)
        {
            panic!("{e}");
        }
        Ok(())
    }

    pub fn list_groups(self, id: i32) -> AccountResult<List<AccountGroupView>> {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let mut result = match account_groups_dsl::account_groups
            .filter(account_groups_dsl::user_id.eq(&id))
            .load::<AccountGroup>(&*conn_guard)
        {
            Ok(result) => result,
            Err(e) => panic!("{e}"),
        };

        Ok(List {
            items: result
                .iter()
                .map(|r| AccountGroupView {
                    name: r.name.to_owned(),
                })
                .collect(),
            total: result.len() as u32,
        })
    }

    pub fn register_account(self, account: &AccountRegister, id: i32) -> AccountResult {
        let conn_guard = match self.connection.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock is poisoned");
                poisoned.into_inner()
            }
        };

        let account = NewAccount {
            name: account.name.as_ref(),
            level: account.level,
            account_groups_id: account.group_id,
            username: account.username.as_ref(),
            password: account.password.as_ref(),
            user_id: id,
        };

        if let Err(e) = diesel::insert_into(accounts::table)
            .values(account)
            .execute(&*conn_guard)
        {
            panic!("{e}");
        }
        Ok(())
    }
}
