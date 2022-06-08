#![allow(dead_code)]
use model::{
    accounts::{
        AccountGroupRegister, AccountGroupView, AccountRegister, AccountView,
        AccountWithPasswordView,
    },
    auth::{AccessToken, LoginRequest, RefreshToken, UserRegister},
    List,
};
use reqwest::StatusCode;

const BASE_URL: &str = "https://api.openpasswd.com";

pub struct OpenPasswdApi {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug)]
pub enum ApiError {
    Reqwest(reqwest::Error),
}

type ApiResult<T = ()> = Result<T, ApiError>;

impl OpenPasswdApi {
    pub fn new() -> OpenPasswdApi {
        OpenPasswdApi {
            access_token: None,
            refresh_token: None,
        }
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.access_token = Some(access_token);
    }

    pub fn set_refresh_token(&mut self, refresh_token: String) {
        self.refresh_token = Some(refresh_token);
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

    pub async fn auth_token(&self, login: LoginRequest) -> ApiResult<AccessToken> {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/token"))
            .json(&login)
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

    pub async fn auth_refresh_token(&self, refresh_token: RefreshToken) -> ApiResult<AccessToken> {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/refresh_token"))
            .json(&refresh_token)
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

    pub async fn auth_logout(&self, refresh_token: RefreshToken) -> ApiResult {
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/logout"))
            .json(&refresh_token)
            .send()
            .await
            .map_err(ApiError::Reqwest)?;

        log::debug!("auth_logout: {}", response.status());
        Ok(())
    }

    pub async fn list_groups(&self) -> ApiResult<List<AccountGroupView>> {
        let access_token = self.access_token.as_ref().unwrap();

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

    pub async fn list_accounts(&self, id: Option<i32>) -> ApiResult<List<AccountView>> {
        let url = if let Some(id) = id {
            format!("{BASE_URL}/api/accounts?group_id={id}")
        } else {
            format!("{BASE_URL}/api/accounts")
        };
        let response = reqwest::Client::new()
            .post(url)
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
        let response = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts"))
            .json(&new_account)
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
        let response = reqwest::Client::new()
            .get(format!("{BASE_URL}/api/accounts/{id}"))
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
