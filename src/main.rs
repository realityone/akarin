#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate ring;
extern crate libc;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate mio;
extern crate pretty_env_logger;
#[macro_use]
extern crate tokio_core;
extern crate transient_hashmap;

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
    let crypto = Ciphers::CHACHA20_POLY1305.init(password);
}
