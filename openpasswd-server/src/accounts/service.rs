use crate::core::cryptography::{AesGcmCipher, Cipher};
use crate::repository::models::account::{NewAccount, NewAccountGroup, NewAccountPassword};
use crate::repository::repositories::accounts_repository::AccountsRepository;
use crate::repository::repositories::users_repository::UsersRepository;
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

    pub async fn register_group(
        self,
        account_group: AccountGroupRegister,
        id: i32,
    ) -> AccountResult<AccountGroupView> {
        let AccountGroupRegister { name } = account_group;
        let account_group = NewAccountGroup { name, user_id: id };

        let account_group = self
            .repository
            .accounts_groups_insert(account_group)
            .await
            .unwrap();

        Ok(AccountGroupView {
            id: account_group.id,
            name: account_group.name,
        })
    }

    pub async fn list_groups(self, user_id: i32) -> AccountResult<List<AccountGroupView>> {
        let result = self.repository.accounts_groups_list(user_id).await;

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

    pub async fn register_account(
        self,
        account: AccountRegister,
        user_id: i32,
    ) -> AccountResult<AccountView> {
        let groups = self
            .repository
            .accounts_groups_find_by_id(account.group_id, user_id)
            .await;

        if groups.is_none() {
            return Err(AccountError::InvalidAccountGroup);
        }

        let master_key = self
            .repository
            .users_find_by_id(user_id)
            .await
            .unwrap()
            .master_key
            .unwrap();
        let cipher = AesGcmCipher::new(&master_key);

        let new_account = NewAccount {
            name: account.name,
            level: account.level,
            account_groups_id: account.group_id,

            user_id,
        };

        let db_account = self.repository.accounts_insert(new_account).await.unwrap();

        let password = cipher.encrypt(&account.password);
        let created_date = chrono::Utc::now().naive_utc();
        let account_password = NewAccountPassword {
            account_id: db_account.id,
            username: account.username,
            password,
            created_date,
        };

        let _ = self
            .repository
            .account_passwords_insert(account_password)
            .await
            .unwrap();

        Ok(AccountView {
            id: db_account.id,
            name: db_account.name,
            group_id: db_account.account_groups_id,
        })
    }

    pub async fn list_accounts(
        self,
        user_id: i32,
        group_id: Option<i32>,
    ) -> AccountResult<List<AccountView>> {
        let result = if let Some(group_id) = group_id {
            self.repository
                .accounts_list_by_group_id(user_id, group_id)
                .await
        } else {
            self.repository.accounts_list(user_id).await
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

    pub async fn get_account(
        self,
        user_id: i32,
        account_id: i32,
    ) -> AccountResult<AccountWithPasswordView> {
        let result = self
            .repository
            .accounts_get_with_passwords_by_account_id(account_id, user_id)
            .await;

        if let Some((account, account_passwords)) = result {
            let master_key = self
                .repository
                .users_find_by_id(user_id)
                .await
                .unwrap()
                .master_key
                .unwrap();
            let cipher = AesGcmCipher::new(&master_key);

            let (username, password) = if let Some(account_password) = account_passwords.last() {
                (
                    Some(account_password.username.to_owned()),
                    Some(cipher.decrypt(&account_password.password)),
                )
            } else {
                (None, None)
            };

            Ok(AccountWithPasswordView {
                id: account.id,
                name: account.name,
                username,
                password,
            })
        } else {
            Err(AccountError::NotFound)
        }
    }
}
