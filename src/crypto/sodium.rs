use sodiumoxide;

use super::Crypto;
use common::error::*;

pub struct Sodium;

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
}
