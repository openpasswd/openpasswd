use std::{cell::RefCell, rc::Rc};

use crate::{
    api::OpenPasswdApi, clipboard::copy_password_to_clipboard, generator::generate_string,
    profile::Profile,
};
use clap::{Args, Subcommand};
use model::accounts::AccountRegister;

#[derive(Debug, Subcommand)]
enum AccountsCommands {
    Get { name: String },
    List { id: Option<i32> },
    Create(Account),
}

#[derive(Debug, Args)]
pub struct Account {
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    group: Option<String>,
    #[clap(short, long, default_value_t = 1)]
    level: i16,
    #[clap(short, long)]
    username: String,
    #[clap(long)]
    generated: bool,
    #[clap(short, long, default_value_t = 16)]
    size: usize,
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Accounts {
    #[clap(subcommand)]
    command: AccountsCommands,
}

impl Accounts {
    pub async fn execute(&self, profile: Rc<RefCell<Profile>>) {
        let api = OpenPasswdApi::new(profile);

        match &self.command {
            AccountsCommands::Get { name } => self.get(api, name).await,
            AccountsCommands::List { id } => self.list(api, id).await,
            AccountsCommands::Create(account) => self.create(api, account).await,
        }
    }

    async fn get(&self, api: OpenPasswdApi, name: &str) {
        let list = api.list_accounts(&None).await.unwrap();
        if let Some(account) = list.items.iter().filter(|a| a.name.as_str() == name).next() {
            let account_with_password = api.get_account(account.id).await.unwrap();

            if let Some(password) = account_with_password.password {
                copy_password_to_clipboard(password, 5);
            }
        }
    }

    async fn list(&self, api: OpenPasswdApi, id: &Option<i32>) {
        let list = api.list_accounts(id).await.unwrap();

        for item in list.items {
            println!("- {}", item.name);
        }
    }

    async fn create(&self, api: OpenPasswdApi, account: &Account) {
        let list = api.list_groups().await.unwrap();
        let group_id = if let Some(group_name) = &account.group {
            if let Some(group) = list
                .items
                .iter()
                .filter(|g| g.name.as_str() == group_name)
                .next()
            {
                group.id
            } else {
                if let Some(group) = list
                    .items
                    .iter()
                    .filter(|g| g.name.contains(group_name))
                    .next()
                {
                    group.id
                } else {
                    panic!("Group specified not found");
                }
            }
        } else {
            if let Some(group) = list.items.first() {
                group.id
            } else {
                panic!("There's no default group");
            }
        };

        println!("Creating Account {}", account.name);
        let password = if account.generated {
            generate_string(account.size)
        } else {
            rpassword::prompt_password("Password: ").unwrap()
        };

        api.register_account(AccountRegister {
            name: account.name.to_owned(),
            group_id,
            level: Some(account.level),
            username: account.username.to_owned(),
            password,
        })
        .await
        .unwrap();
    }
}
