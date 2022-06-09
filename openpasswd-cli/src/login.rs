use crate::api::OpenPasswdApi;
use crate::profile::Profile;
use clap::Args;
use model::auth::{LoginRequest, RefreshTokenType};
use std::{
    cell::RefCell,
    io::{BufRead, Write},
    rc::Rc,
};

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
    pub fn new() -> Login {
        Login { clean: false }
    }

    pub async fn execute(&self, profile: Rc<RefCell<Profile>>) {
        let api = OpenPasswdApi::new(profile.clone());
        let email = {
            match profile.borrow().email() {
                Some(email) if !self.clean => email.to_owned(),
                _ => read_prompt_input("Email: ").unwrap(),
            }
        };

        let device_name = {
            match profile.borrow().device_name() {
                Some(device_name) if !self.clean => device_name.to_owned(),
                _ => read_prompt_input("Device Name: ").unwrap(),
            }
        };

        let password = rpassword::prompt_password("Password: ").unwrap();

        api.auth_token(LoginRequest {
            email: email.clone(),
            password,
            device_name: Some("XXX".to_owned()),
            refresh_token: Some(RefreshTokenType::Token),
        })
        .await
        .unwrap();

        {
            let mut profile = profile.borrow_mut();
            profile.set_email(email);
            profile.set_device_name(device_name);
        }
    }
}
