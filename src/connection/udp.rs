use crate::log_err;

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

    pub fn open_localhost(&mut self, port: u16) -> bool {
        if self.socket.is_some() {
            return false;
        }

        let addr = format!("127.0.0.1:{}", port);

        match UdpSocket::bind(addr.as_str()) {
            Ok(sock) => {
                self.socket = Some(sock);

                return true;
            }
            Err(e) => {
                log_err!("[{}] UDPソケットのバインドに失敗しました: {}", self.name, e);
                return false;
            }
        }
    }
}
