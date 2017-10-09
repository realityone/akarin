use std::fmt;
use std::net::SocketAddr;

use futures::Future;
use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
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
pub struct AkarinServer<'a> {
    tun: Device,
    udp: UdpSocket,

    crypto: &'a Crypto,

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

impl<'a> AkarinServer<'a> {
    fn new<'b>(tun: Device, crypto: &'a Crypto, udp: UdpSocket, configuration: &'b ServerConfiguration) -> Self {
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

impl<'a> Server for AkarinServer<'a> {
    fn serve(self, mut core: Core, mut handle: Handle) -> Result<()> {
        let mut buff = vec![0u8; 1500];
        let s = self.tun.read_dgram(&mut buff);
        core.run(s);
        Ok(())
    }
}
