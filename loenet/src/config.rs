use std::net::{SocketAddr, IpAddr, Ipv4Addr};

pub struct Config {
    pub websocket_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            websocket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8765),
        }
    }

}

impl Config {

    pub fn new(port: u16) -> Self {
        Config {
            websocket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port),
        }
    }
}
