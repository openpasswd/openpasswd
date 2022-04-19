use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use rsa::{pkcs8::EncodePrivateKey, PublicKey};
use serde::Serialize;

use self::dto::Account;

mod dto;

#[derive(Serialize)]
struct List<T> {
    items: Vec<T>,
    total: u32,
}

pub async fn list() -> impl IntoResponse {
    let list = List {
        items: vec![Account {
            id: String::from("47523942-bc63-11ec-8422-0242ac120002"),
            username: String::from("alcantarafox@yahoo.com.br"),
            name: String::from("netflix"),
            password: None,
        }],
        total: 1,
    };

    (StatusCode::OK, Json(list))
}

pub async fn get(Path(id): Path<String>) -> impl IntoResponse {
    let stop_watch = std::time::Instant::now();

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = rsa::RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = rsa::RsaPublicKey::from(&priv_key);
    println!("Generate keys: {}", stop_watch.elapsed().as_secs_f32());

    let pem = priv_key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
    println!("private key:\n{}", *pem);

    let stop_watch = std::time::Instant::now();
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(
            &mut rng,
            rsa::PaddingScheme::new_pkcs1v15_encrypt(),
            &data[..],
        )
        .expect("failed to encrypt");
    println!("Encrypt: {}", stop_watch.elapsed().as_secs_f32());

    let account = Account {
        id,
        username: String::from("alcantarafox@yahoo.com.br"),
        name: String::from("netflix"),
        password: Some(base64::encode(enc_data)),
    };

    let stop_watch = std::time::Instant::now();
    // Decrypt
    let dec_data = priv_key
        .decrypt(
            rsa::PaddingScheme::new_pkcs1v15_encrypt(),
            &base64::decode(account.password.as_ref().unwrap()).unwrap(),
        )
        .expect("failed to decrypt");
    println!("Decrypt: {}", stop_watch.elapsed().as_secs_f32());

    println!("{}", String::from_utf8(dec_data).unwrap());

    (StatusCode::OK, Json(account))
}
