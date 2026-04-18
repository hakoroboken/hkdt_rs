use hkdt_rs::connection::serial::Serial;
use hkdt_rs::arm_bot::ArmBot;

fn main()
{
    let mut serial = Serial::new("/dev/ttyACM0", 115200).unwrap();

    let mut arm_bot = ArmBot::new();
    
    loop {
        // let data = serial.read(&mut buf).unwrap_or_default();

        // // let angle_data = (buf[1] as i16) << 8 | (buf[2] as i16);
        // println!("Size:{}", data);
        // println!("Received: [{}]", buf[0]);

        let s = serial.read_str();

        match s {
            Some(line) =>{
                arm_bot.update(line);
                println!("ArmBot Data: {:?}", arm_bot.motors[1]);
            },
            None => println!("Failed to read from serial port"),
        }
    }
}