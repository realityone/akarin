mod sys;
pub mod device;

pub use self::device::{create, Device};

#[cfg(test)]
mod tests;
