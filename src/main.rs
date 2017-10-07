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

fn init<C>(crypto: &mut C) -> Result<()>
    where C: crypto::Cipher {
    info!("Initializing crypto `{}`", C::name());
    if let Err(e) = crypto.init() {
        info!("Failed to init crypto: `{}`", C::name());
        return Err(e);
    }
    info!("Initializing components succeed: crypto=`{}`", C::name());
    Ok(())
}

fn main() {
    // setup logger
    pretty_env_logger::init().unwrap();

    let ref mut crypto = crypto::sodium::Sodium {};
    init(crypto).unwrap();
}
