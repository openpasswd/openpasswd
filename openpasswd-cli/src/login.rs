use clap::Args;
use model::auth::{LoginRequest, RefreshTokenType};

use std::io::{BufRead, Write};

use crate::api::OpenPasswdApi;

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
    pub async fn execute(&self) {
        let email = read_prompt_input("Email: ").unwrap();
        let password = rpassword::prompt_password("Password: ").unwrap();

        let result = OpenPasswdApi::auth_token(LoginRequest {
            email,
            password,
            device_name: Some("XXX".to_owned()),
            refresh_token: Some(RefreshTokenType::Token),
        })
        .await
        .unwrap();

        if let Some(proj_dir) =
            directories::ProjectDirs::from("com", "openpasswd", "openpasswd-cli")
        {
            print!("{:?}", proj_dir.config_dir());
        }

        println!("{}", result.access_token);
    }
}
