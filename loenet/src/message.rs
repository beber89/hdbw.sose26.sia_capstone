use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub dst_app: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub data: Vec<serde_json::Value>,  // Arbitrary JSON
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header {{dst_app: \"{}\"}}",
            self.dst_app
        )
    }
}


impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

