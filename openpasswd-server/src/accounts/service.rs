use crate::repository::models::account_group::{AccountGroup, NewAccount, NewAccountGroup};
use crate::repository::repositories::accounts_repository::AccountsRepository;
use crate::repository::schema::account_groups;
use crate::repository::schema::account_groups::dsl as account_groups_dsl;
use crate::repository::schema::accounts;
use log::warn;
// use crate::orm::schema::accounts::dsl as accounts_dsl;
use openpasswd_model::accounts::{AccountGroupRegister, AccountGroupView, AccountRegister};
use openpasswd_model::List;

use super::dto::accounts_error::AccountResult;

pub struct AccountService<T>
where
    T: AccountsRepository,
{
    repository: T,
}

impl<T> AccountService<T>
where
    T: AccountsRepository,
{
    pub fn new(repository: T) -> AccountService<T> {
        AccountService { repository }
    }

    pub fn register_group(self, account_group: &AccountGroupRegister, id: i32) -> AccountResult {
        let account_group = NewAccountGroup {
            name: account_group.name.as_ref(),
            user_id: id,
        };

        self.repository.accounts_groups_insert(account_group);
        Ok(())
    }

    pub fn list_groups(self, user_id: i32) -> AccountResult<List<AccountGroupView>> {
        let result = self.repository.accounts_groups_list(user_id);

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
        let account = NewAccount {
            name: account.name.as_ref(),
            level: account.level,
            account_groups_id: account.group_id,
            username: account.username.as_ref(),
            password: account.password.as_ref(),
            user_id: id,
        };

        self.repository.accounts_insert(account);
        Ok(())
    }
}
