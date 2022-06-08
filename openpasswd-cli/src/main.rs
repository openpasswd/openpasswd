extern crate copypasta;

use accounts::Accounts;
use clap::{Parser, Subcommand};
use generator::Generator;
use login::Login;

mod accounts;
mod api;
mod generator;
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
    Login(Login),
    Account(Accounts),
    Generator(Generator),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Login(login) => login.execute().await,
        Commands::Account(account) => account.execute().await,
        Commands::Generator(generator) => generator.execute(),
    }
}
