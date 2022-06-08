use clap::Args;
use model::auth::{LoginRequest, RefreshTokenType};

use std::io::{BufRead, Write};

use crate::{api::OpenPasswdApi, profile::Profile};

fn read_prompt_input(prompt: &str) -> std::io::Result<String> {
    print!("{}", prompt);
    std::io::stdout().flush()?;

    let mut value = String::new();
    std::io::stdin().lock().read_line(&mut value)?;

    Ok(value.trim().to_owned())
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
pub struct Login {
    #[clap(short, long)]
    clean: bool,
}

impl Login {
    pub async fn execute(&self) {
        let mut profile = Profile::new();
        let api = OpenPasswdApi::new();
        let email = match profile.email() {
            Some(email) if !self.clean => email.to_owned(),
            _ => read_prompt_input("Email: ").unwrap(),
        };

        let password = rpassword::prompt_password("Password: ").unwrap();

        let result = api
            .auth_token(LoginRequest {
                email: email.clone(),
                password,
                device_name: Some("XXX".to_owned()),
                refresh_token: Some(RefreshTokenType::Token),
            })
            .await
            .unwrap();

        profile.set_email(email);
        profile.set_access_token(result.access_token);

        if let Some(refresh_token) = result.refresh_token {
            profile.set_refresh_token(refresh_token);
        }
    }
}
