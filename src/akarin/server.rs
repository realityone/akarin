use std::{fmt, io, mem};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::net::SocketAddr;
use std::ops::Range;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;
use tokio_core::reactor::{Core, Handle};
use transient_hashmap::TransientHashMap;

use super::{Server, State, new_buf};
use super::configuration::ServerConfiguration;
use common::error::*;
use crypto::Crypto;
use transport::network::{IPV4_HEADER_LEN, IPv4Header};
use tun::os::tokio::Device;

type ClientId = u32;
type ClientToken = u64;
type ClientMetadata = (ClientToken, SocketAddr);

#[derive(Debug)]
pub struct AkarinServer<'a> {
    tun: Device,
    udp: UdpSocket,

    crypto: &'a Crypto,

    clients: ClientStorage,

    tun_buf: Vec<u8>,
    udp_buf: Vec<u8>,

    state: State,
}

pub struct ClientStorage {
    id_set: HashSet<ClientId>,
    storage: TransientHashMap<ClientId, ClientMetadata>,
}

impl ClientStorage {
    pub fn new(id_range: Range<ClientId>, lifetime: u32) -> Self {
        ClientStorage {
            id_set: HashSet::from_iter(id_range.into_iter()),
            storage: TransientHashMap::new(lifetime),
        }
    }

    pub fn available_ids(&self) -> Vec<ClientId> {
        self.id_set.iter().map(|i| *i).collect()
    }

    pub unsafe fn reserve_id(&mut self, id: ClientId) -> Result<ClientId> {
        if !self.id_set.remove(&id) {
            return Err(ErrorKind::ReserveClientIDFailed.into());
        }
        Ok(id)
    }

    fn next_id(&self) -> Option<&ClientId> {
        self.id_set.iter().nth(0)
    }

    pub fn insert_client(&mut self, meta: &ClientMetadata) -> Result<ClientId> {
        let id = {
            match self.next_id() {
                Some(id) => id.clone(),
                None => return Err(ErrorKind::MaxClientExceed.into()),
            }
        };
        self.id_set.remove(&id);
        self.storage.insert(id, *meta);

        Ok(id)
    }

    pub fn refresh_client(&mut self, id: ClientId, meta: &ClientMetadata) -> Result<()> {
        if !self.compare_client(id, meta) {
            return Err(ErrorKind::NoSuchClientID.into());
        }

        self.storage.insert(id, *meta);
        Ok(())
    }

    pub fn get(&mut self, id: ClientId) -> Option<&ClientMetadata> {
        self.storage.get(&id)
    }

    pub fn compare_client(&mut self, id: ClientId, meta: &ClientMetadata) -> bool {
        if self.id_set.contains(&id) {
            return false;
        }

        if let Some(ref stored) = self.storage.get(&id) {
            if stored == &meta {
                return true;
            }
        }
        false
    }

    pub fn remove_client(&mut self, id: ClientId) {
        self.id_set.insert(id);
        self.storage.remove(&id);
    }

    pub fn prune(&mut self) {
        for id in self.storage.prune().iter() {
            self.id_set.insert(*id);
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

            clients: ClientStorage::new(0..255, configuration.client_timeout.unwrap_or(60)),

            tun_buf: new_buf(configuration.mtu.unwrap_or(1432) as usize),
            udp_buf: new_buf(configuration.mtu.unwrap_or(1432) as usize),

            state: State::Down,
        }
    }
}

impl<'a> Future for AkarinServer<'a> {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            if try_nb!(self.tun.read(&mut self.tun_buf)) < *IPV4_HEADER_LEN {
                return Err(io::ErrorKind::UnexpectedEof.into());
            };

            let header = {
                let mut header_bytes = [0u8; 20];
                header_bytes.copy_from_slice(&self.tun_buf[..*IPV4_HEADER_LEN]);
                IPv4Header::from(header_bytes)
            };

            let client_id = header.destination_address;
            if let Some(&(token, sockaddr)) = self.clients.get(client_id) {}
        }
    }
}

impl<'a> Server for AkarinServer<'a> {
    fn serve(mut self, mut core: Core, handle: Handle) -> Result<()> {
        core.run(self);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_client_storage() {
        let reserved = 2..10;
        let ref mut us = ClientStorage::new(0..255, 60);
        for r in reserved {
            unsafe {
                us.reserve_id(r).unwrap();
            }
        }

        let available_ids = us.available_ids();
        for id in available_ids {
            assert!(id < 2 || id >= 10);
        }

        let client = (123u64, SocketAddr::from_str("192.168.1.1:80").unwrap());
        let cid = us.insert_client(&client).unwrap();
        us.refresh_client(cid, &client).unwrap();
        assert!(us.refresh_client(cid + 1, &client).is_err());
        assert!(us.compare_client(cid, &client));
        assert_eq!(us.get(cid).unwrap(), &client);
        assert!(us.get(cid + 1).is_none());
    }
}
