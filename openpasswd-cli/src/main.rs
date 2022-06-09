extern crate copypasta;

use accounts::Accounts;
use clap::{Parser, Subcommand};
use generator::Generator;
use groups::Groups;
use login::Login;
use profile::Profile;
use std::{cell::RefCell, rc::Rc};

mod accounts;
mod api;
mod clipboard;
mod generator;
mod groups;
mod login;
mod profile;

/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[clap(name = "openpasswd-cli")]
#[clap(about = "The official OpenPasswd CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Login(Login),
    Account(Accounts),
    Group(Groups),
    Generator(Generator),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let profile = Rc::new(RefCell::new(Profile::new()));

    if profile.borrow().is_token_expired() {
        Login::new().execute(profile.clone()).await
    }

    match args.command {
        // Commands::Login(login) => login.execute(profile).await,
        Commands::Account(account) => account.execute(profile).await,
        Commands::Group(group) => group.execute(profile).await,
        Commands::Generator(generator) => generator.execute(),
    }
}
