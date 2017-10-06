#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
extern crate libc;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[macro_use]
extern crate ioctl_sys;

mod common;
mod tun;

fn main() {
    println!("Hello, world!");
}
