#![allow(unused)]
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes128Gcm, Aes256Gcm, Key, Nonce};
use rand::distributions::{Alphanumeric, Standard};
use rand::prelude::Distribution;
use rand::Rng;
use uuid::Uuid;

pub trait Cipher {
    fn encrypt(&self, value: &str) -> Vec<u8>;
    fn decrypt(&self, value: &[u8]) -> String;
}

pub struct AesGcmCipher {
    cipher: Aes256Gcm,
}

impl AesGcmCipher {
    pub fn new(key: &str) -> AesGcmCipher {
        let key = Key::from_slice(key.as_bytes());
        let cipher = Aes256Gcm::new(key);

        AesGcmCipher { cipher }
    }
}

impl Cipher for AesGcmCipher {
    fn encrypt(&self, value: &str) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let randomness: Vec<u8> = Standard.sample_iter(&mut rng).take(12).collect();
        let nonce = Nonce::from_slice(&randomness); // 12-bytes; unique per message
        let ciphertext = self
            .cipher
            .encrypt(nonce, value.as_bytes())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        let mut password: Vec<u8> = randomness;
        password.extend(&ciphertext);

        password
    }

    fn decrypt(&self, value: &[u8]) -> String {
        let n = &value[0..12];
        let nn = Nonce::from_slice(n);

        let plaintext = self
            .cipher
            .decrypt(nn, &value[12..])
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        String::from_utf8(plaintext).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{AesGcmCipher, Cipher};

    const ID: &str = "67dcca6627454cff81cc811e2a4a17b9";
    #[test]
    fn is_encrypting() {
        let cipher = AesGcmCipher::new(ID);
        let value = cipher.encrypt("hello_world");
        let result = cipher.decrypt(&value);
        assert_eq!("hello_world", result);
    }
}
