pub mod sodium;

pub use self::sodium::Sodium;

use common::error::*;

pub trait Crypto {
    fn name(&self) -> String;

    fn encrypt_inplace(&self, message: &mut [u8]) -> Result<()>;
    fn decrypt_inplace(&self, cipher_text: &mut [u8]) -> Result<()>;
}
