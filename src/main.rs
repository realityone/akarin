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

use crypto::Crypto;

fn init_crypto(password: &str) -> Box<crypto::Crypto> {
    use crypto::sodium::Sodium;
    info!("Initializing crypto: `{}`", Sodium::name());

    let crypto =
        Box::new(Sodium::new(password)
            .map_err(|e| info!("Failed to init crypto: `{}`, {}", Sodium::name(), e))
            .unwrap());
    info!("Initializing crypto succeed: `{}`", Sodium::name());
    crypto
}

fn main() {
    // setup logger
    pretty_env_logger::init().unwrap();

    let password = "realityone";
    let crypto = init_crypto(password);
}
