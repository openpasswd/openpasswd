extern crate copypasta;

use accounts::Accounts;
use clap::{Parser, Subcommand};
use copypasta::{ClipboardContext, ClipboardProvider};
use login::Login;

mod accounts;
mod api;
mod login;

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
    Test,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Login(login) => login.execute().await,
        Commands::Account(account) => account.execute(),
        Commands::Test => test(),
    }
}

fn test() {
    let the_string = "Test Sample!".to_owned();

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(the_string).unwrap();

    let seconds = 5;
    println!("Password ready do be pasted for {seconds} seconds");
    std::thread::sleep(std::time::Duration::from_secs(seconds));
    ctx.set_contents("".to_owned()).unwrap();
    println!("Clipboard unset");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
