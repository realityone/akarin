use sodiumoxide;

use sodiumoxide::randombytes;
use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::stream::salsa208;
use sodiumoxide::crypto::onetimeauth::poly1305;

use super::Crypto;
use common::error::*;

pub struct Sodium {
    key: salsa208::Key,
}

impl Sodium {
    pub fn new(password: &str) -> Result<Self> {
        if !sodiumoxide::init() {
            return Err(ErrorKind::InitCryptoFailed.into());
        }

        let key = salsa208::Key(hash::sha256::hash(password.as_bytes()).0);
        Ok(Self { key: key })
    }

    #[inline]
    pub fn name() -> String {
        "sodium".to_string()
    }
}

impl Crypto for Sodium {
    fn name(&self) -> String {
        Self::name()
    }

    fn encrypt_inplace(&self, message: &mut [u8]) -> Result<()> {
        if message.len() < 32 {
            return Err(ErrorKind::InvalidMessage.into());
        }

        let ref mut nonce = salsa208::Nonce([0u8; 8]);
        randombytes::randombytes_into(&mut nonce.0);

        let cipher_text = message;
        salsa208::stream_xor_inplace(cipher_text, nonce, &self.key);

        let ref auth_tag = {
            let mut key_buf = [0u8; 32];
            key_buf.clone_from_slice(&cipher_text[..32]);
            let ref poly_key = poly1305::Key(key_buf);
            poly1305::authenticate(&cipher_text[32..], poly_key)
        };
        cipher_text[16..16 + auth_tag.0.len()].clone_from_slice(auth_tag.as_ref());

        Ok(())
    }

    fn decrypt_inplace(&self, cipher_text: &mut [u8]) -> Result<()> {
        let ref nonce = {
            let mut nonce_buff = [0u8; 8];
            nonce_buff.clone_from_slice(&cipher_text[..8]);
            salsa208::Nonce(nonce_buff)
        };

        let ref subkey = {
            let mut key_buff = [0u8; 32];
            key_buff.clone_from_slice(&salsa208::stream(32, nonce, &self.key));
            poly1305::Key(key_buff)
        };

        let ref auth_tag = {
            let mut tag_buff = [0u8; 16];
            let tag_buff_len = tag_buff.len();
            tag_buff.clone_from_slice(&cipher_text[16..16 + tag_buff_len]);
            poly1305::Tag(tag_buff)
        };

        if !poly1305::verify(auth_tag, &cipher_text[32..], subkey) {
            return Err(ErrorKind::InvalidMessage.into());
        }

        let message = cipher_text;
        salsa208::stream_xor_inplace(message, nonce, &self.key);

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        let origin_message = br#"# akarin

[![Build Status](https://travis-ci.org/realityone/akarin.svg?branch=master)](https://travis-ci.org/realityone/akarin)

Lightweight and stateless IP tunnel.
"#;
        let crypto = Sodium::new("realityone").unwrap();

        let cipher_text = {
            let mut message = origin_message.clone();
            crypto.encrypt_inplace(&mut message);
            message
        };

        let plain_text = {
            let mut message = cipher_text.clone();
            crypto.decrypt_inplace(&mut message);
            message
        };
    }
}