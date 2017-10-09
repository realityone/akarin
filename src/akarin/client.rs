use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};

use super::{Client, State, new_buff};
use super::configuration::ClientConfiguration;
use common::error::*;
use crypto::Crypto;
use tun::os::tokio::Device;

#[derive(Debug)]
pub struct AkarinClient<'a> {
    tun: Device,
    udp: UdpSocket,

    crypto: &'a Crypto,

    tun_buff: Vec<u8>,
    udp_buff: Vec<u8>,

    state: State,
}

impl<'a> AkarinClient<'a> {
    fn new<'d>(tun: Device, crypto: &'a Crypto, udp: UdpSocket, configuration: &'d ClientConfiguration) -> Self {
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

impl<'a> Client for AkarinClient<'a> {
    fn connect(self, mut core: Core, mut handle: Handle) -> Result<()> {
        unimplemented!()
    }
}
