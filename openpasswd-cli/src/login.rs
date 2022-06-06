use clap::Args;

use std::io::{BufRead, Write};

fn read_prompt_input(prompt: &str) -> std::io::Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;

    let mut value = String::new();
    std::io::stdin().lock().read_line(&mut value)?;

    Ok(value.trim().to_owned())
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Login {}

impl Login {
    pub fn execute(&self) {
        let email = read_prompt_input("Email: ").unwrap();
        let password = rpassword::prompt_password("Password: ").unwrap();

        println!("email: {email} - password: {password}");
    }
}
