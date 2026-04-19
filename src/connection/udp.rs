use std::net::UdpSocket;
use std::time::{Duration, Instant};

pub struct UdpHandler {
    name: String,
    socket: Option<UdpSocket>,
    timer: Instant,
    period: Duration,
    destination_addr: Option<String>,
    recv_destination_addr: Option<String>,
}

impl UdpHandler {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            socket: None,
            timer: Instant::now(),
            period: Duration::from_millis(1),
            destination_addr: None,
            recv_destination_addr: None,
        }
    }
}
