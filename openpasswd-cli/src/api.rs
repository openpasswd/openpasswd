#![allow(dead_code)]
use crate::profile::Profile;
use std::{cell::RefCell, rc::Rc};

use model::{
    accounts::{
        AccountGroupRegister, AccountGroupView, AccountRegister, AccountView,
        AccountWithPasswordView,
    },
    auth::{AccessToken, LoginRequest, RefreshToken, RefreshTokenType, UserRegister},
    List,
};
use reqwest::StatusCode;

const BASE_URL: &str = "https://api.openpasswd.com";

pub struct OpenPasswdApi {
    profile: Rc<RefCell<Profile>>,
}

#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
}

type ApiResult<T = ()> = Result<T, ApiError>;

impl OpenPasswdApi {
    pub fn new(profile: Rc<RefCell<Profile>>) -> OpenPasswdApi {
        OpenPasswdApi { profile }
    }

    pub async fn auth_register(&self, user: UserRegister) -> ApiResult {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/user"))
            .json(&user)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::CREATED {
            Ok(())
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn login(&self) -> ApiResult {
        let (email, device_name) = {
            let profile = self.profile.borrow();
            (
                profile.email().unwrap().to_owned(),
                profile.device_name().unwrap().to_owned(),
            )
        };
        let password = rpassword::prompt_password("Password: ").unwrap();

        self.auth_token(LoginRequest {
            email,
            password,
            device_name: Some(device_name),
            refresh_token: Some(RefreshTokenType::Token),
        })
        .await
        .unwrap();

        Ok(())
    }

    pub async fn auth_token(&self, login: LoginRequest) -> ApiResult {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/token"))
            .json(&login)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::OK {
            let result: AccessToken = response.json().await.map_err(ApiError::Reqwest)?;
            self.profile
                .borrow_mut()
                .set_tokens(Some(result.access_token), result.refresh_token);
            Ok(())
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn auth_refresh_token(&self) -> ApiResult<AccessToken> {
        let refresh_token = self.profile.borrow().refresh_token().unwrap().to_owned();
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/refresh_token"))
            .json(&RefreshToken { refresh_token })
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::OK {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn auth_logout(&self) -> ApiResult {
        let (access_token, refresh_token) = {
            let profile = self.profile.borrow();
            (
                profile.access_token().unwrap().to_owned(),
                profile.refresh_token().unwrap().to_owned(),
            )
        };
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/logout"))
            .bearer_auth(access_token)
            .json(&RefreshToken { refresh_token })
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        log::debug!("auth_logout: {}", response.status());
        Ok(())
    }

    pub async fn list_groups(&self) -> ApiResult<List<AccountGroupView>> {
        let access_token = self.profile.borrow().access_token().unwrap().to_owned();

        let response = reqwest::Client::new()
            .get(format!("{BASE_URL}/api/accounts/groups"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::OK {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn register_group(
        &self,
        new_account_group: AccountGroupRegister,
    ) -> ApiResult<AccountGroupView> {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts/groups"))
            .json(&new_account_group)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::CREATED {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn list_accounts(&self, id: &Option<i32>) -> ApiResult<List<AccountView>> {
        let access_token = self.profile.borrow().access_token().unwrap().to_owned();
        let url = if let Some(id) = id {
            format!("{BASE_URL}/api/accounts?group_id={id}")
        } else {
            format!("{BASE_URL}/api/accounts")
        };
        let response = reqwest::Client::new()
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::OK {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn register_account(&self, new_account: AccountRegister) -> ApiResult<AccountView> {
        let access_token = self.profile.borrow().access_token().unwrap().to_owned();
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts"))
            .json(&new_account)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::CREATED {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }

    pub async fn get_account(&self, id: i32) -> ApiResult<AccountWithPasswordView> {
        let access_token = self.profile.borrow().access_token().unwrap().to_owned();
        let response = reqwest::Client::new()
            .get(format!("{BASE_URL}/api/accounts/{id}"))
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        if response.status() == StatusCode::OK {
            let result = response.json().await.map_err(ApiError::Reqwest)?;
            Ok(result)
        } else {
            let text = response.text().await.map_err(ApiError::Reqwest)?;
            panic!("{text}");
        }
    }
}
