use std::net::{SocketAddr, IpAddr, Ipv4Addr};

pub struct Config {
    pub websocket_addr: SocketAddr,
    pub http_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            websocket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8765),
            http_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000),
        }
    }
}