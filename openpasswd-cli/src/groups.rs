use std::{cell::RefCell, rc::Rc};

use clap::{Args, Subcommand};
use model::accounts::AccountGroupRegister;

use crate::{api::OpenPasswdApi, profile::Profile};

#[derive(Debug, Subcommand)]
enum GroupsCommands {
    List,
    Create { name: String },
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Groups {
    #[clap(subcommand)]
    command: GroupsCommands,
}

impl Groups {
    pub async fn execute(&self, profile: Rc<RefCell<Profile>>) {
        let api = OpenPasswdApi::new(profile);

        match &self.command {
            GroupsCommands::List => self.list(api).await,
            GroupsCommands::Create { name } => self.create(api, name).await,
        }
    }

    async fn list(&self, api: OpenPasswdApi) {
        let list = api.list_groups().await.unwrap();

        for item in list.items {
            println!("- {}", item.name);
        }
    }

    async fn create(&self, api: OpenPasswdApi, name: &str) {
        api.register_group(AccountGroupRegister {
            name: name.to_owned(),
        })
        .await
        .unwrap();
    }
}
