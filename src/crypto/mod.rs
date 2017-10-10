#[cfg(feature = "libsodium")]
pub mod salsa2012;

use std::fmt::Debug;

use common::error::*;

pub trait Crypto: Debug {
    fn name(&self) -> String;

    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub enum Ciphers {
    #[cfg(feature = "libsodium")]
    SALSA2012,
}

impl Ciphers {
    pub fn init(self, password: &str) -> Box<Crypto> {
        match self {
            #[cfg(feature = "libsodium")]
            Ciphers::SALSA2012 => salsa2012::init_crypto(password),
        }
    }
}

impl<'a> From<&'a str> for Ciphers {
    fn from(name: &str) -> Self {
        match name {
            #[cfg(feature = "libsodium")]
            "libsodium" => Ciphers::SALSA2012,
            #[cfg(feature = "libsodium")]
            _ => Ciphers::SALSA2012,
        }
    }
}
