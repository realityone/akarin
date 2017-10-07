mod sys;
pub mod device;

pub use self::device::{Device, create};

#[cfg(test)]
mod tests;
