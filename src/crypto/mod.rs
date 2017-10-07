pub mod sodium;

pub use self::sodium::Sodium;

use common::error::*;

pub trait Cipher {
    fn init(&mut self) -> Result<()>;
    fn name() -> String;
}
