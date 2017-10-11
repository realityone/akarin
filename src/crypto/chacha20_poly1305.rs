use std::fmt;

use ring::{aead, digest, pbkdf2, rand};
use ring::rand::SecureRandom;

use super::Crypto;
use common::error::*;

pub fn init_crypto(password: &str) -> Box<Crypto> {
    info!("Initializing crypto: `{}`", ChaCha20Poly1305::name());

    let crypto = Box::new(ChaCha20Poly1305::new(password.as_bytes())
                              .map_err(|e| error!("Failed to init crypto: `{}`, {}", ChaCha20Poly1305::name(), e))
                              .unwrap());

    info!("Initializing crypto succeed: `{}`", ChaCha20Poly1305::name());
    crypto
}

pub struct ChaCha20Poly1305 {
    sealing_key: aead::SealingKey,
    opening_key: aead::OpeningKey,
    random: rand::SystemRandom,
}

impl fmt::Debug for ChaCha20Poly1305 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChaCha20Poly1305 {{ sealing_key, opening_key, random }}")
    }
}

impl ChaCha20Poly1305 {
    pub fn new(password: &[u8]) -> Result<Self> {
        let mut hashed_key = [0; digest::SHA256_OUTPUT_LEN];
        pbkdf2::derive(&digest::SHA256, 1024, &[], password, &mut hashed_key);

        let sealing_key = aead::SealingKey::new(&aead::CHACHA20_POLY1305, &hashed_key)?;
        let opening_key = aead::OpeningKey::new(&aead::CHACHA20_POLY1305, &hashed_key)?;

        Ok(ChaCha20Poly1305 {
               sealing_key,
               opening_key,
               random: rand::SystemRandom::new(),
           })
    }

    #[inline]
    pub fn name() -> String {
        "chacha20_poly1305".to_string()
    }
}

impl Crypto for ChaCha20Poly1305 {
    fn name(&self) -> String {
        Self::name()
    }

    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>> {
        let tag_length = self.sealing_key.algorithm().tag_len();
        let nonce_len = self.sealing_key.algorithm().nonce_len();

        let mut nonce = vec![0u8; nonce_len];
        self.random.fill(&mut nonce)?;

        let cipher_len = nonce_len + message.len() + tag_length;
        let mut cipher_text = Vec::with_capacity(cipher_len);
        cipher_text.extend_from_slice(&nonce);
        cipher_text.extend_from_slice(message);
        cipher_text.resize(cipher_len, 0u8);

        aead::seal_in_place(&self.sealing_key, &nonce, &[], &mut cipher_text[nonce_len..], tag_length)?;
        Ok(cipher_text)
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let tag_length = self.sealing_key.algorithm().tag_len();
        let nonce_len = self.sealing_key.algorithm().nonce_len();

        let message_len = cipher_text.len() - nonce_len - tag_length;
        let (nonce, mut message) = {
            let (nonce, message) = cipher_text.split_at(nonce_len);
            (nonce, message.to_vec())
        };

        aead::open_in_place(&self.opening_key, &nonce, &[], 0, &mut message)?;
        message.resize(message_len, 0);
        Ok(message)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt_and_decrypt() {
        let origin_message = br#"# akarin

Lightweight and stateless IP tunnel.
"#
                             .to_vec();
        let crypto = ChaCha20Poly1305::new(b"realityone").unwrap();

        let cipher_text = {
            let message = origin_message.to_vec();
            crypto.encrypt(&message).unwrap()
        };

        let plain_text = {
            let message = cipher_text.to_vec();
            crypto.decrypt(&message).unwrap()
        };

        assert_eq!(origin_message, plain_text);
    }
}
