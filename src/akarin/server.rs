use std::{fmt, io};
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
use transient_hashmap::TransientHashMap;

use super::{new_buff, Server, State};
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
        ClientStorage {
            storage: TransientHashMap::new(lifetime),
        }
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

impl<'a> Future for AkarinServer<'a> {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let s = try_nb!(self.tun.read(&mut self.tun_buff));
            println!("{:?}, {:?}", s, self.tun_buff);
        }
    }
}

impl<'a> Server for AkarinServer<'a> {
    fn serve(mut self, mut core: Core, mut handle: Handle) -> Result<()> {
        core.run(self);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use tun::os::create;

    use crypto::salsa2012::Salsa2012;
    use tokio_core::net::UdpSocket;
    use tokio_core::reactor::{Core, Handle};
    use tun::Tun;
    use tun::configuration;

    #[test]
    fn test_server_read() {
        let mut config = configuration::Configuration::default();

        let addr = Ipv4Addr::from_str("10.0.0.2").unwrap();
        let netmask = Ipv4Addr::from_str("255.255.255.0").unwrap();
        let destination = Ipv4Addr::from_str("10.0.0.1").unwrap();
        let mtu = 1480;

        config.name("utun6")
              .address(addr)
              .netmask(netmask)
              .destination(destination)
              .mtu(mtu)
              .up();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let tun = Device::new(create(&config).unwrap(), &handle).unwrap();
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let udp = UdpSocket::bind(&addr, &handle).unwrap();
        let crypto = Salsa2012::new(b"realityone").unwrap();

        let c = ServerConfiguration::default();

        let s = AkarinServer::new(tun, &crypto, udp, &c);
        s.serve(core, handle);
    }
}
