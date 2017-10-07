pub mod sodium;

pub use self::sodium::Sodium;

use common::error::*;

pub trait Crypto {
    fn init(&mut self) -> Result<()>;
    fn name() -> String;

    fn set_password(&mut self, password: &str) -> Result<()>;
}
