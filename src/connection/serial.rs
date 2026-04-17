use serialport;
use termion::input::TermRead;

use crate::log_err;

pub struct Serial
{
    port: Box<dyn serialport::SerialPort>,
}

impl Serial {
    pub fn new(port_name : &str, baud_rate: u32)->Option<Self>
    {
        match serialport::new(port_name, baud_rate).timeout(std::time::Duration::from_millis(1000)).open() {
            Ok(port) => Some(Self { port }),
            Err(e) => {
                log_err!("Failed to open serial port {}: {}", port_name, e);
                None
            }
        }
    }

    pub fn write(&mut self, data : &[u8])->bool
    {
        match self.port.write(data) {
            Ok(_) => true,
            Err(e) => {
                log_err!("Failed to write to serial port: {}", e);
                false
            }
        }
    }

    pub fn read(&mut self, buffer: &mut [u8])->Option<usize>
    {
        match self.port.read(buffer) {
            Ok(bytes_read) => Some(bytes_read),
            Err(e) => {
                log_err!("Failed to read from serial port: {}", e);
                None
            }
        }
    }

    pub fn read_str(&mut self)->Option<String>
    {
        match self.port.read_line() {
            Ok(line) =>{
                line
            },
            Err(_e) =>{
                // log_err!("Failed to read from serial port: {}", e)
                None
            },
        }
    }
}