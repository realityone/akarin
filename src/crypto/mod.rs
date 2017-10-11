pub mod chacha20_poly1305;

use std::fmt::Debug;

use common::error::*;

pub trait Crypto: Debug {
    fn name(&self) -> String;

    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, cipher_text: &[u8]) -> Result<Vec<u8>>;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Ciphers {
    CHACHA20_POLY1305,
}

impl Ciphers {
    pub fn init(self, password: &str) -> Box<Crypto> {
        match self {
            Ciphers::CHACHA20_POLY1305 => chacha20_poly1305::init_crypto(password),
        }
    }
}

impl<'a> From<&'a str> for Ciphers {
    fn from(name: &str) -> Self {
        match name {
            "chacha20_poly1305" => Ciphers::CHACHA20_POLY1305,
            _ => Ciphers::CHACHA20_POLY1305,
        }
    }
}
