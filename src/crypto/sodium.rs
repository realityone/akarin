use sodiumoxide;

use sodiumoxide::crypto::hash;

use super::Crypto;
use common::error::*;

#[derive(Default)]
pub struct Sodium {
    key: Option<[u8; 32]>,
}

impl Crypto for Sodium {
    fn init(&mut self) -> Result<()> {
        if !sodiumoxide::init() {
            return Err(ErrorKind::InitCryptoFailed.into());
        }

        Ok(())
    }

    fn name() -> String {
        "sodium".to_string()
    }

    fn set_password(&mut self, password: &str) -> Result<()> {
        if let Some(_) = self.key {
            return Err(ErrorKind::CryptoPasswordAlreadySetted.into());
        }

        self.key = Some(hash::sha256::hash(password.as_bytes()).0);
        Ok(())
    }
}
