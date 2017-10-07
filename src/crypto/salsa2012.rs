use sodiumoxide;

use sodiumoxide::randombytes;
use sodiumoxide::crypto::hash;
use sodiumoxide::crypto::stream::salsa2012;

use super::Crypto;
use common::error::*;

pub fn init_crypto(password: &str) -> Box<Crypto> {
    info!("Initializing crypto: `{}`", Salsa2012::name());

    let crypto =
        Box::new(Salsa2012::new(password.as_bytes())
                     .map_err(|e| error!("Failed to init crypto: `{}`, {}", Salsa2012::name(), e))
                     .unwrap());

    info!("Initializing crypto succeed: `{}`", Salsa2012::name());
    crypto
}

#[derive(Debug)]
pub struct Salsa2012 {
    key: salsa2012::Key,
}

impl Salsa2012 {
    pub fn new(password: &[u8]) -> Result<Self> {
        if !sodiumoxide::init() {
            return Err(ErrorKind::InitCryptoFailed.into());
        }

        let key = salsa2012::Key(hash::sha256::hash(password).0);
        Ok(Self { key: key })
    }

    #[inline]
    pub fn name() -> String {
        "salsa2012".to_string()
    }
}

impl Crypto for Salsa2012 {
    fn name(&self) -> String {
        Self::name()
    }

    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>> {
        let mut cipher_text = vec![0u8; message.len() + salsa2012::NONCEBYTES];

        let ref nonce = {
            let mut nonce_buff = [0u8; salsa2012::NONCEBYTES];
            randombytes::randombytes_into(&mut nonce_buff);
            salsa2012::Nonce(nonce_buff)
        };

        cipher_text[..salsa2012::NONCEBYTES].clone_from_slice(nonce.as_ref());
        cipher_text[salsa2012::NONCEBYTES..].clone_from_slice(message);
        salsa2012::stream_xor_inplace(&mut cipher_text[salsa2012::NONCEBYTES..], nonce, &self.key);

        Ok(cipher_text)
    }

    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>> {
        let mut message = vec![0u8; cipher_text.len() - salsa2012::NONCEBYTES];

        let ref nonce = {
            let mut nonce_buff = [0u8; 8];
            nonce_buff.clone_from_slice(&cipher_text[..8]);
            salsa2012::Nonce(nonce_buff)
        };

        message.clone_from_slice(&cipher_text[salsa2012::NONCEBYTES..]);
        salsa2012::stream_xor_inplace(&mut message, nonce, &self.key);

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
        let crypto = Salsa2012::new(b"realityone").unwrap();

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
