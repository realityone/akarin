use sodiumoxide;

use super::Cipher;
use common::error::*;

pub struct Sodium;

impl Cipher for Sodium {
    fn init(&mut self) -> Result<()> {
        if !sodiumoxide::init() {
            return Err(ErrorKind::InitCryptoFailed.into());
        }

        Ok(())
    }

    fn name() -> String {
        "sodium".to_string()
    }
}
