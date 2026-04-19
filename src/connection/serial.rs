use serialport;
use termion::input::TermRead;

use crate::log_err;

pub struct Serial {
    name: String,
    port: Box<dyn serialport::SerialPort>,
}

impl Serial {
    pub fn new(name: &str, port_name: &str, baud_rate: u32) -> Option<Self> {
        match serialport::new(port_name, baud_rate)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
        {
            Ok(port) => Some(Self {
                name: name.to_string(),
                port,
            }),
            Err(_e) => {
                log_err!("[{}]シリアルポートを開けませんでした。:{}", name, port_name);
                None
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) -> bool {
        match self.port.write(data) {
            Ok(_) => true,
            Err(e) => {
                log_err!(
                    "[{}]シリアル通信の書き込みに失敗しました。: {}",
                    self.name,
                    e
                );
                false
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Option<usize> {
        match self.port.read(buffer) {
            Ok(bytes_read) => Some(bytes_read),
            Err(e) => {
                log_err!(
                    "[{}]シリアル通信の読み込みに失敗しました。: {}",
                    self.name,
                    e
                );
                None
            }
        }
    }

    pub fn read_str(&mut self) -> Option<String> {
        match self.port.read_line() {
            Ok(line) => line,
            Err(e) => {
                log_err!(
                    "[{}]シリアル通信の読み込みに失敗しました。: {}",
                    self.name,
                    e
                );
                None
            }
        }
    }
}
