use std::net::UdpSocket;

use super::{Client, State, new_buff};
use super::configuration::ClientConfiguration;
use common::error::*;
use crypto::Crypto;
use tun::Tun;

#[derive(Debug)]
pub struct AkarinClient<'a, 'b, 'c> {
    tun: &'a Tun,
    crypto: &'b Crypto,
    udp: &'c UdpSocket,

    tun_buff: Vec<u8>,
    udp_buff: Vec<u8>,

    state: State,
}

impl<'a, 'b, 'c> AkarinClient<'a, 'b, 'c> {
    fn new<'d>(tun: &'a Tun, crypto: &'b Crypto, udp: &'c UdpSocket, configuration: &'d ClientConfiguration) -> Self {
        AkarinClient {
            tun,
            crypto,
            udp,

            tun_buff: new_buff(configuration.mtu.unwrap_or(1432) as usize),
            udp_buff: new_buff(configuration.mtu.unwrap_or(1432) as usize),

            state: State::Down,
        }
    }
}

impl<'a, 'b, 'c> Client for AkarinClient<'a, 'b, 'c> {
    fn connect(&mut self) -> Result<()> {
        unimplemented!()
    }
}
