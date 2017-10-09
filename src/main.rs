#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate mio;
extern crate libc;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate tokio_core;
#[cfg(feature = "libsodium")]
extern crate sodiumoxide;
#[macro_use]
extern crate error_chain;
extern crate transient_hashmap;
extern crate pretty_env_logger;

#[cfg(unix)]
#[macro_use]
extern crate ioctl_sys;

mod tun;
mod akarin;
mod common;
mod crypto;
mod transport;

use crypto::Ciphers;

fn main() {
    // setup logger
    pretty_env_logger::init().unwrap();

    let password = "realityone";
    let crypto = Ciphers::SALSA2012.init(password);
}
