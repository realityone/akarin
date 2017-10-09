pub mod server;
pub mod client;
pub mod configuration;

use tokio_core::reactor::{Core, Handle};

use common::error::*;

#[derive(Debug)]
pub enum State {
    Running,
    Down,
}

pub const AKARIN_ZERO_BYTES: usize = 32;
pub const AKARIN_OVERHEAD_LEN: usize = 24;
pub const AKARIN_PACKET_OFFSET: usize = 8;
pub const AKARIN_USERTOKEN_LEN: usize = 8;


pub fn new_buff(mtu: usize) -> Vec<u8> {
    vec![0u8; mtu + AKARIN_ZERO_BYTES + AKARIN_USERTOKEN_LEN]
}

pub trait Server {
    fn serve(self, mut core: Core, mut handle: Handle) -> Result<()>;
}

pub trait Client {
    fn connect(self, mut core: Core, mut handle: Handle) -> Result<()>;
}
