use std::net::SocketAddr;

#[derive(Clone, Default, Debug)]
pub struct ClientConfiguration {
    pub server_address: Option<SocketAddr>,
    pub mtu: Option<i32>,
}


#[derive(Clone, Default, Debug)]
pub struct ServerConfiguration {
    pub mtu: Option<i32>,
    pub client_timeout: Option<u32>,
}

impl ClientConfiguration {
    pub fn server_address(&mut self, value: SocketAddr) -> &mut Self {
        self.server_address = Some(value);
        self
    }

    pub fn mtu(&mut self, value: i32) -> &mut Self {
        self.mtu = Some(value);
        self
    }
}

impl ServerConfiguration {
    pub fn mtu(&mut self, value: i32) -> &mut Self {
        self.mtu = Some(value);
        self
    }

    pub fn client_timeout(&mut self, value: u32) -> &mut Self {
        self.client_timeout = Some(value);
        self
    }
}
