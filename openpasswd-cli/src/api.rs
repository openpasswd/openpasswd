#![allow(dead_code)]
#![allow(unused_variables)]
use model::{
    accounts::{
        AccountGroupRegister, AccountGroupView, AccountRegister, AccountView,
        AccountWithPasswordView,
    },
    auth::{AccessToken, LoginRequest, RefreshToken, UserRegister},
    List,
};

const BASE_URL: &str = "https://api.openpasswd.com";

pub struct OpenPasswdApi {}

type ApiResult<T = ()> = Result<T, reqwest::Error>;

impl OpenPasswdApi {
    // async post<B, R>(path: string, body: B): Promise<Response<R>> {
    // async send<R>(url: RequestInfo, init: RequestInit): Promise<Response<R>> {
    pub async fn auth_register(user: UserRegister) -> ApiResult {
        reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/user"))
            .json(&user)
            .send()
            .await?;
        Ok(())
    }

    pub async fn auth_token(login: LoginRequest) -> ApiResult<AccessToken> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/token"))
            .json(&login)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    pub async fn auth_refresh_token(refresh_token: RefreshToken) -> ApiResult<AccessToken> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/refresh_token"))
            .json(&refresh_token)
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    pub async fn auth_logout(refresh_token: RefreshToken) -> ApiResult {
        reqwest::Client::new()
            .post(format!("{BASE_URL}/api/auth/logout"))
            .json(&refresh_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn list_groups() -> ApiResult<List<AccountGroupView>> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts/groups"))
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    pub async fn register_group(
        new_account_group: AccountGroupRegister,
    ) -> ApiResult<AccountGroupView> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts/groups"))
            .json(&new_account_group)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    pub async fn list_accounts(id: i32) -> ApiResult<List<AccountView>> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts"))
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }

    pub async fn register_account(new_account: AccountRegister) -> ApiResult<AccountView> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts"))
            .json(&new_account)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    pub async fn get_account(id: i32) -> ApiResult<AccountWithPasswordView> {
        let result = reqwest::Client::new()
            .post(format!("{BASE_URL}/api/accounts/{id}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(result)
    }
}
