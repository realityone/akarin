use std::fmt;
use std::net::SocketAddr;

use tokio_core::net::UdpSocket;
use transient_hashmap::TransientHashMap;

use super::{Server, State, new_buff};
use super::configuration::ServerConfiguration;
use common::error::*;
use crypto::Crypto;
use tun::os::tokio::Device;

type ClientId = u8;
type ClientToken = u64;
type ClientMetadata = (ClientToken, SocketAddr);

#[derive(Debug)]
pub struct AkarinServer<'a, 'b, 'c> {
    tun: &'a Device,
    crypto: &'b Crypto,
    udp: &'c UdpSocket,

    clients: ClientStorage,

    tun_buff: Vec<u8>,
    udp_buff: Vec<u8>,

    state: State,
}

pub struct ClientStorage {
    storage: TransientHashMap<ClientId, ClientMetadata>,
}

impl ClientStorage {
    fn new(lifetime: u32) -> Self {
        ClientStorage { storage: TransientHashMap::new(lifetime) }
    }
}

impl fmt::Debug for ClientStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.storage.iter()).finish()
    }
}

impl<'a, 'b, 'c> AkarinServer<'a, 'b, 'c> {
    fn new<'d>(tun: &'a Device, crypto: &'b Crypto, udp: &'c UdpSocket, configuration: &'d ServerConfiguration)
               -> Self {
        AkarinServer {
            tun,
            crypto,
            udp,

            clients: ClientStorage::new(configuration.client_timeout.unwrap_or(60)),

            tun_buff: new_buff(configuration.mtu.unwrap_or(1432) as usize),
            udp_buff: new_buff(configuration.mtu.unwrap_or(1432) as usize),

            state: State::Down,
        }
    }
}

impl<'a, 'b, 'c> Server for AkarinServer<'a, 'b, 'c> {
    fn serve(&mut self) -> Result<()> {
        unimplemented!()
    }
}
