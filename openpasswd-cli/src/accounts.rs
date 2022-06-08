use clap::{Args, Subcommand};

use crate::{api::OpenPasswdApi, profile::Profile};

#[derive(Debug, Subcommand)]
enum AccountsCommands {
    List,
    Create { name: String },
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Accounts {
    #[clap(subcommand)]
    command: AccountsCommands,
}

impl Accounts {
    pub async fn execute(&self) {
        match &self.command {
            AccountsCommands::List => self.list().await,
            AccountsCommands::Create { name } => println!("Create Group: {name}"),
        }
    }

    async fn list(&self) {
        let profile = Profile::new();
        let mut api = OpenPasswdApi::new();
        if let Some(access_token) = profile.access_token() {
            api.set_access_token(access_token.to_owned());
        }

        let list = api.list_groups().await.unwrap();

        for item in list.items {
            println!("- {}", item.name);
        }
    }
}
