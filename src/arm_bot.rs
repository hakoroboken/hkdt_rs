pub const ARM_BOT_MAX_HORIZONTAL_POSITION: f32 = 0.0;
pub const ARM_BOT_MIN_HORIZONTAL_POSITION: f32 = -145.0;
pub const ARM_BOT_MAX_VERTICAL_POSITION: f32 = 0.0;
pub const ARM_BOT_MIN_VERTICAL_POSITION: f32 = -110.0;
pub const ARM_BOT_MAX_HAND_POSITION: f32 = 25.0;
pub const ARM_BOT_MIN_HAND_POSITION: f32 = -25.0;

#[derive(Debug, Clone, Copy)]
pub struct MotorData {
    first_count: i16,
    count: i16,
    revolution: i16,
    pub position: f32,
    pub velocity: i16,
    pub current: i16,
}

impl MotorData {
    pub fn new() -> Self {
        Self {
            first_count: 0,
            count: -1,
            revolution: 0,
            position: 0.0,
            velocity: 0,
            current: 0,
        }
    }
}

pub struct ArmBot {
    motors: [MotorData; 3],
}

impl ArmBot {
    pub fn new() -> Self {
        let mut motors = [MotorData::new(); 3];

        for i in 0..3 {
            motors[i] = MotorData::new();
        }

        Self { motors }
    }

    pub fn create_send_buffer(
        &self,
        wheel1: f64,
        wheel2: f64,
        wheel3: f64,
        horizon: i16,
        vertical: i16,
        hand: i16,
    ) -> [u8; 8] {
        let mut send_data = [0_u8; 8];
        send_data[0] = pwm_to_byte(wheel1);
        send_data[1] = pwm_to_byte(wheel2);
        send_data[2] = pwm_to_byte(wheel3);
        send_data[3] = current_to_byte(horizon);
        send_data[4] = current_to_byte(vertical);
        send_data[5] = current_to_byte(hand);
        send_data[6] = b'\r';
        send_data[7] = b'\n';

        send_data
    }

    pub fn update_sensor(&mut self, read_line: String) {
        let mut byte_vec = Vec::<u8>::new();
        for i in read_line.split_whitespace() {
            byte_vec.push(i.parse::<u8>().unwrap());
        }
        let data = byte_vec.as_slice();

        if data.len() < 17 {
        } else {
            let mut motors = [MotorData::new(); 3];
            for i in 0..3 {
                motors[i].count = (data[i * 6] as i16) << 8 | (data[i * 6 + 1] as i16);
                self.motors[i].velocity = (data[i * 6 + 2] as i16) << 8 | (data[i * 6 + 3] as i16);
                self.motors[i].current = (data[i * 6 + 4] as i16) << 8 | (data[i * 6 + 5] as i16);

                if motors[i].count == -1 || self.motors[i].count == -1 {
                    self.motors[i].count = motors[i].count;
                    self.motors[i].first_count = motors[i].count;
                    self.motors[i].revolution = 0;
                    self.motors[i].position = 0.0;
                } else {
                    let diff = self.motors[i].count - motors[i].count;

                    if diff > 4096 {
                        self.motors[i].revolution += 1;
                    } else if diff < -4096 {
                        self.motors[i].revolution -= 1;
                    }

                    self.motors[i].count = motors[i].count;

                    self.motors[i].position = self.motors[i].revolution as f32
                        + (motors[i].count - self.motors[i].first_count) as f32 / 8192.0;
                }
            }
        }
    }

    pub fn get_horizontal_motor(&self) -> MotorData {
        self.motors[0]
    }

    pub fn get_vertical_motor(&self) -> MotorData {
        self.motors[1]
    }

    pub fn get_hand_motor(&self) -> MotorData {
        self.motors[2]
    }
}

fn pwm_to_byte(pwm: f64) -> u8 {
    (pwm * 127.0 + 127.0) as u8
}

fn current_to_byte(current: i16) -> u8 {
    ((current / 10000) as f64 * 127.0 + 127.0) as u8
}
