use std::fs::{self, create_dir};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    email: Option<String>,
    device_name: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl Profile {
    pub fn new() -> Profile {
        if let Some(project_dirs) =
            directories::ProjectDirs::from("com", "openpasswd", "openpasswd-cli")
        {
            let file = project_dirs.config_dir().join(".profile");
            if file.exists() {
                let json = fs::read_to_string(file).unwrap();
                return serde_json::from_str(&json).unwrap();
            }
        }

        Profile {
            email: None,
            device_name: None,
            access_token: None,
            refresh_token: None,
        }
    }

    fn save(&self) {
        let json = serde_json::to_string(self).unwrap();
        if let Some(project_dirs) =
            directories::ProjectDirs::from("com", "openpasswd", "openpasswd-cli")
        {
            let dir = project_dirs.config_dir();
            if !dir.exists() {
                create_dir(dir).unwrap();
            }

            let file = dir.join(".profile");
            fs::write(file, json.as_bytes()).unwrap();
        }
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
        self.save();
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn set_device_name(&mut self, device_name: String) {
        self.device_name = Some(device_name);
        self.save();
    }

    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    pub fn set_tokens(&mut self, access_token: Option<String>, refresh_token: Option<String>) {
        self.access_token = access_token;
        self.refresh_token = refresh_token;
        self.save();
    }

    #[allow(dead_code)]
    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = Some(access_token);
        self.save();
    }

    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    #[allow(dead_code)]
    pub fn set_refresh_token(&mut self, refresh_token: String) {
        self.refresh_token = Some(refresh_token);
        self.save();
    }

    #[allow(dead_code)]
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }
}
