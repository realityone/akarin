use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};

use super::{Client, State, new_buf};
use super::configuration::ClientConfiguration;
use common::error::*;
use crypto::Crypto;
use tun::os::tokio::Device;

#[derive(Debug)]
pub struct AkarinClient<'a> {
    tun: Device,
    udp: UdpSocket,

    crypto: &'a Crypto,

    tun_buf: Vec<u8>,
    udp_buf: Vec<u8>,

    state: State,
}

impl<'a> AkarinClient<'a> {
    fn new<'d>(tun: Device, crypto: &'a Crypto, udp: UdpSocket, configuration: &'d ClientConfiguration) -> Self {
        AkarinClient {
            tun,
            crypto,
            udp,

            tun_buf: new_buf(configuration.mtu.unwrap_or(1432) as usize),
            udp_buf: new_buf(configuration.mtu.unwrap_or(1432) as usize),

            state: State::Down,
        }
    }
}

impl<'a> Client for AkarinClient<'a> {
    fn connect(self, mut core: Core, handle: Handle) -> Result<()> {
        unimplemented!()
    }
}
