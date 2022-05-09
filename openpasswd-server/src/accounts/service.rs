use crate::core::cryptography::{AesGcmCipher, Cipher};
use crate::repository::models::account::{NewAccount, NewAccountGroup, NewAccountPassword};
use crate::repository::repositories::accounts_repository::AccountsRepository;
use crate::repository::repositories::users_repository::UsersRepository;
// use crate::orm::schema::accounts::dsl as accounts_dsl;
use openpasswd_model::accounts::{
    AccountGroupRegister, AccountGroupView, AccountRegister, AccountView, AccountWithPasswordView,
};
use openpasswd_model::List;

use super::dto::accounts_error::{AccountError, AccountResult};

pub struct AccountService<T>
where
    T: AccountsRepository + UsersRepository,
{
    repository: T,
}

impl<T> AccountService<T>
where
    T: AccountsRepository + UsersRepository,
{
    pub fn new(repository: T) -> AccountService<T> {
        AccountService { repository }
    }

    pub fn register_group(
        self,
        account_group: &AccountGroupRegister,
        id: i32,
    ) -> AccountResult<AccountGroupView> {
        let account_group = NewAccountGroup {
            name: account_group.name.as_ref(),
            user_id: id,
        };

        let account_group = self
            .repository
            .accounts_groups_insert(account_group)
            .unwrap();

        Ok(AccountGroupView {
            id: account_group.id,
            name: account_group.name,
        })
    }

    pub fn list_groups(self, user_id: i32) -> AccountResult<List<AccountGroupView>> {
        let result = self.repository.accounts_groups_list(user_id);

        Ok(List {
            items: result
                .iter()
                .map(|r| AccountGroupView {
                    id: r.id,
                    name: r.name.to_owned(),
                })
                .collect(),
            total: result.len() as u32,
        })
    }

    pub fn register_account(
        self,
        account: &AccountRegister,
        user_id: i32,
    ) -> AccountResult<AccountView> {
        let groups = self
            .repository
            .accounts_groups_find_by_id_and_user_id(account.group_id, user_id);

        if groups.is_none() {
            return Err(AccountError::InvalidAccountGroup);
        }

        let master_key = self
            .repository
            .users_find_by_id(user_id)
            .unwrap()
            .master_key
            .unwrap();
        let cipher = AesGcmCipher::new(&master_key);

        let new_account = NewAccount {
            name: account.name.as_ref(),
            level: account.level,
            account_groups_id: account.group_id,

            user_id,
        };

        let db_account = self.repository.accounts_insert(new_account).unwrap();

        let password = cipher.encrypt(&account.password);
        let created_date: std::time::SystemTime = chrono::Utc::now().into();
        let account_password = NewAccountPassword {
            account_id: db_account.id,
            username: account.username.as_ref(),
            password: &password,
            created_date,
        };

        let _ = self
            .repository
            .account_passwords_insert(account_password)
            .unwrap();

        Ok(AccountView {
            id: db_account.id,
            name: db_account.name,
            group_id: db_account.account_groups_id,
        })
    }

    pub fn list_accounts(
        self,
        user_id: i32,
        group_id: Option<i32>,
    ) -> AccountResult<List<AccountView>> {
        let result = if let Some(group_id) = group_id {
            self.repository
                .accounts_list_by_user_id_and_group_id(user_id, group_id)
        } else {
            self.repository.accounts_list_by_user_id(user_id)
        };

        Ok(List {
            items: result
                .iter()
                .map(|r| AccountView {
                    id: r.id,
                    name: r.name.to_owned(),
                    group_id: r.account_groups_id,
                })
                .collect(),
            total: result.len() as u32,
        })
    }

    pub fn get_account(
        self,
        user_id: i32,
        account_id: i32,
    ) -> AccountResult<AccountWithPasswordView> {
        let result = self
            .repository
            .accounts_get_with_password_by_id_and_user_id(account_id, user_id);

        let master_key = self
            .repository
            .users_find_by_id(user_id)
            .unwrap()
            .master_key
            .unwrap();
        let cipher = AesGcmCipher::new(&master_key);

        let account = result.ok_or(AccountError::NotFound)?;
        if let Some(last_password) = account.passwords.last() {
            Ok(AccountWithPasswordView {
                id: account.id,
                name: account.name,
                username: last_password.username.to_owned(),
                password: cipher.decrypt(&last_password.password),
            })
        } else {
            Ok(AccountWithPasswordView {
                id: account.id,
                name: account.name,
                username: "".to_owned(),
                password: "".to_owned(),
            })
        }
    }
}
