mod sys;
pub mod device;

pub use self::device::create;

#[cfg(test)]
mod tests;
