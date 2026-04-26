use crate::{log_err, log_warn};

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

    pub fn open_auto_address(&mut self, period : u64)
    {
        if self.socket.is_some()
        {
            log_warn!("[{}]ソケットが作成済みです。.", self.name);
            return;
        }

        match UdpSocket::bind("0.0.0.0:0") {
            Ok(socket)=>{
                socket.connect("8.8.8.8:80").unwrap();

                socket.set_read_timeout(Some(Duration::from_millis(period))).unwrap();
                self.socket = Some(socket)
            }
            Err(e)=>{
                log_err!("[{}]UDPソケットのバインドに失敗しました : {}", self.name, e);
            }
        }
    }

    /// 設定したアドレスでソケットを作成する
    /// * `addr` - ポートもまとめたアドレス。例えば「192.168.0.50:64202」と入力する
    /// * `period` - 受信のタイムアウトを設定する。例えば`1`にすると受信待機してから１ｍｓ受信できないとエラーを吐くようにする
    pub fn open_set_address(&mut self, addr : &str, period : u64)
    {
        if self.socket.is_some()
        {
            log_warn!("[{}] ソケットが作成済みです。", self.name);
            return;
        }

        match UdpSocket::bind(addr) {
            Ok(socket)=>{
                socket.set_read_timeout(Some(Duration::from_millis(period))).unwrap();
                self.socket = Some(socket)
            }
            Err(e)=>{
                log_err!("[{}] UDPソケットのバインドに失敗しました : {}", self.name, e);
            }
        }
    }

    /// 送信相手のアドレスを登録する
    /// * `addr` - 相手のアドレス。例えば「192.168.0.50:64201」
    pub fn set_destination(&mut self, addr : &str)
    {
        self.destination_addr = Some(addr.to_string());
    }

    /// 送信周期を決める
    /// * `period` - ミリ秒で設定する。例えば1msごとに送信したい場合は`1`を入れる
    pub fn set_send_period(&mut self, period : u64)
    {
        self.period = Duration::from_millis(period);
    }


    /// 登録したアドレスに値を送信する
    /// * `buf` - 8bitの正の整数の配列のポインタ
    pub fn send(&mut self, buf : &[u8])
    {
        if self.timer.elapsed() >= self.period
        {
            match &self.socket {
                Some(sock)=>{
                    match self.destination_addr.clone()
                    {
                        Some(dest)=>{
                            match sock.send_to(buf, dest.as_str()) {
                                Ok(_s)=>{
                                }
                                Err(e)=>{
                                    log_err!("[{}] 送信相に失敗しました : {}", self.name, e);
                                }
                            }
                        }
                        None=>{
                            log_warn!("[{}]送信相手が設定されていません。", self.name);
                        }
                    }
                }
                None=>{
                    log_warn!("[{}]ソケットが作成されていません", self.name);
                }
            }

            self.timer = Instant::now();
        }
    }

    pub fn recv(&mut self)->Option<String>
    {
        let mut buf = [0_u8; 1024];
        match &self.socket {
            Some(socket)=>{
                match socket.recv_from(&mut buf) {
                    Ok((size, dest_addr))=>{
                        let get_data = &buf[..size];
                        
                        self.recv_destination_addr = Some(dest_addr.to_string());

                        let str = String::from_utf8_lossy(&get_data).to_string();

                        Some(str)
                    }
                    Err(e)=>{
                        log_err!("[{}] 受信に失敗しました : {}", self.name, e);

                        None
                    }
                }
            }
            None=>{
                log_warn!("[{}] ソケットが作成されていません", self.name);

                None
            }
        }
    }

    /// データを送ってきた相手のアドレスを取得する。
    /// 受信後でないと`None`が返ってくる
    pub fn who(&self)->String
    {
        if self.recv_destination_addr.is_some()
        {
            self.recv_destination_addr.clone().unwrap()
        }
        else {
            String::from("None")
        }
    }
}
