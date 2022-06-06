use clap::{Args, Subcommand};

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
    pub fn execute(&self) {
        match &self.command {
            AccountsCommands::List => println!("List Groups"),
            AccountsCommands::Create { name } => println!("Create Group: {name}"),
        }
    }
}
