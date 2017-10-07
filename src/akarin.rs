use std::net::SocketAddr;

use tun::Tun;
use crypto::Crypto;
use transport::Transport;

#[derive(Debug)]
pub enum Mode {
    Server,
    Client,
}

#[derive(Debug)]
pub enum State {
    Running,
    Down,
}

#[derive(Debug)]
pub struct AkarinServerCtx<'a> {
    tun: &'a Tun,
    crypto: &'a Crypto,
    transport: &'a Transport,

    tun_buff: Vec<u8>,
    udp_buff: Vec<u8>,

    state: State,
}

#[derive(Debug)]
pub struct AkarinClientCtx<'a> {
    remote_addr: SocketAddr,

    tun: &'a Tun,
    crypto: &'a Crypto,
    transport: &'a Transport,

    tun_buff: Vec<u8>,
    udp_buff: Vec<u8>,

    state: State,
}
