#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "linux")]
pub mod linux;
pub mod configuration;

#[cfg(target_os = "macos")]
pub use self::macos as os;
#[cfg(target_os = "macos")]
pub use self::macos::create;

#[cfg(target_os = "linux")]
pub use self::linux as os;
#[cfg(target_os = "linux")]
pub use self::linux::create;

pub use self::configuration::Configuration;

use std::net::Ipv4Addr;
use std::io::{Write, Read};

use common::error::*;

pub trait Tun: Read + Write {
    fn name(&self) -> &str;

    fn address(&self) -> Result<Ipv4Addr>;
    fn set_address(&mut self, value: Ipv4Addr) -> Result<()>;

    fn broadcast(&self) -> Result<Ipv4Addr>;
    fn set_broadcast(&mut self, value: Ipv4Addr) -> Result<()>;

    fn destination(&self) -> Result<Ipv4Addr>;
    fn set_destination(&mut self, value: Ipv4Addr) -> Result<()>;

    fn netmask(&self) -> Result<Ipv4Addr>;
    fn set_netmask(&mut self, value: Ipv4Addr) -> Result<()>;

    fn mtu(&self) -> Result<i32>;
    fn set_mtu(&mut self, value: i32) -> Result<()>;

    fn flags(&self) -> Result<i16>;
    fn set_flags(&mut self, value: i16) -> Result<()>;

    fn set_enabled(&mut self, value: bool) -> Result<()>;
}
