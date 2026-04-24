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
