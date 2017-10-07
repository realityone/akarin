#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate libc;
#[macro_use]
extern crate error_chain;
extern crate sodiumoxide;
extern crate pretty_env_logger;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[macro_use]
extern crate ioctl_sys;

mod tun;
mod common;
mod crypto;
mod transport;

use common::error::*;

fn init_crypto<C>(crypto: &mut C, password: &str) -> Result<()>
where
    C: crypto::Crypto,
{
    info!("Initializing crypto: `{}`", C::name());
    if let Err(e) = crypto.init() {
        info!("Failed to init crypto: `{}`, {}", C::name(), e);
        return Err(e);
    }
    if let Err(e) = crypto.set_password(password) {
        info!("Failed to set crypto password: `{}`, {}", C::name(), e);
        return Err(e);
    }
    info!("Initializing crypto succeed: `{}`", C::name());
    Ok(())
}

fn main() {
    // setup logger
    pretty_env_logger::init().unwrap();

    let password = "realityone";
    let ref mut crypto = crypto::sodium::Sodium::default();
    init_crypto(crypto, password).unwrap();
}
