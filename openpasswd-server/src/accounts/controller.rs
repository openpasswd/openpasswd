use std::collections::HashMap;

use super::{dto::accounts_error::AccountResult, service::AccountService};
use crate::{auth::dto::claims::Claims, core::validator::ValidatedJson, repository::Repository};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use openpasswd_model::accounts::{AccountGroupRegister, AccountRegister};

// use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
// use openpasswd_model::{accounts::AccountView, List};
// use rsa::{pkcs8::EncodePrivateKey, PublicKey};

pub async fn register_group(
    claims: Claims,
    ValidatedJson(account_groups): ValidatedJson<AccountGroupRegister>,
    Extension(repository): Extension<Repository>,
) -> AccountResult<impl IntoResponse> {
    let account_service = AccountService::new(repository);
    let account_group = account_service
        .register_group(account_groups, claims.sub)
        .await?;
    Ok((StatusCode::CREATED, Json(account_group)))
}

pub async fn list_groups(
    claims: Claims,
    Extension(repository): Extension<Repository>,
) -> AccountResult<impl IntoResponse> {
    let account_service = AccountService::new(repository);
    let result = account_service.list_groups(claims.sub).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub async fn register_account(
    claims: Claims,
    ValidatedJson(account): ValidatedJson<AccountRegister>,
    Extension(repository): Extension<Repository>,
) -> AccountResult<impl IntoResponse> {
    let account_service = AccountService::new(repository);
    let account = account_service
        .register_account(account, claims.sub)
        .await?;
    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn list_accounts(
    claims: Claims,
    Query(params): Query<HashMap<String, String>>,
    Extension(repository): Extension<Repository>,
) -> AccountResult<impl IntoResponse> {
    let account_service = AccountService::new(repository);
    let group_id = if let Some(group_id) = params.get("group_id") {
        if let Ok(group_id) = group_id.parse::<i32>() {
            Some(group_id)
        } else {
            None
        }
    } else {
        None
    };
    let result = account_service.list_accounts(claims.sub, group_id).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub async fn get_account(
    claims: Claims,
    Path(account_id): Path<i32>,
    Extension(repository): Extension<Repository>,
) -> AccountResult<impl IntoResponse> {
    let account_service = AccountService::new(repository);
    let result = account_service.get_account(claims.sub, account_id).await?;
    Ok((StatusCode::OK, Json(result)))
}

// pub async fn list() -> impl IntoResponse {
//     let list = List {
//         items: vec![AccountView {
//             id: String::from("47523942-bc63-11ec-8422-0242ac120002"),
//             username: String::from("alcantarafox@yahoo.com.br"),
//             name: String::from("netflix"),
//             password: None,
//         }],
//         total: 1,
//     };

//     (StatusCode::OK, Json(list))
// }

// pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
//     let stop_watch = std::time::Instant::now();

//     let mut rng = rand::thread_rng();
//     let bits = 2048;
//     let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//     let pub_key = rsa::RsaPublicKey::from(&priv_key);
//     println!("Generate keys: {}", stop_watch.elapsed().as_secs_f32());

//     let pem = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
//     println!("private key:\n{}", *pem);

//     let stop_watch = std::time::Instant::now();
//     // Encrypt
//     let data = b"hello world";
//     let enc_data = pub_key
//         .encrypt(
//             &mut rng,
//             rsa::PaddingScheme::new_pkcs1v15_encrypt(),
//             &data[..],
//         )
//         .expect("failed to encrypt");
//     println!("Encrypt: {}", stop_watch.elapsed().as_secs_f32());

//     let account = AccountView {
//         id,
//         username: String::from("alcantarafox@yahoo.com.br"),
//         name: String::from("netflix"),
//         password: Some(base64::encode(enc_data)),
//     };

//     let stop_watch = std::time::Instant::now();
//     // Decrypt
//     let dec_data = priv_key
//         .decrypt(
//             rsa::PaddingScheme::new_pkcs1v15_encrypt(),
//             &base64::decode(account.password.as_ref().unwrap()).unwrap(),
//         )
//         .expect("failed to decrypt");
//     println!("Decrypt: {}", stop_watch.elapsed().as_secs_f32());

//     println!("{}", String::from_utf8(dec_data).unwrap());

//     (StatusCode::OK, Json(account))
// }
