use crate::kinematics::common::UnitPosition;
use crate::log_err;
use std::collections::HashMap;

pub struct Omni
{
    max_velocity_ : f32,
    unit_positions_ : UnitPosition,
    output : HashMap<String, f32>
}

impl Omni {
    pub fn new(unit_data: UnitPosition, max_velocity: f32)-> Self
    {
        Self {
            max_velocity_: max_velocity,
            unit_positions_: unit_data,
            output: HashMap::new()
        }
    }

    pub fn compute(&mut self, x_vec : f32, y_vec: f32, rotation_vec : f32)
    {
        self.output.clear();
        let mut max_velocity = 0.0;

        for (pos_name, pos) in self.unit_positions_.data.iter()
        {
            let name = String::from(pos_name);

            // 中心からの距離
            let distance = pos.squared_distance().sqrt();

            let vel = x_vec * pos.yaw.cos() + y_vec * pos.yaw.sin() + rotation_vec * distance;

            if vel > max_velocity
            {
                max_velocity = vel;
            }

            self.output.insert(name, vel);
        }

        if max_velocity > self.max_velocity_
        {
            for (_name, vel) in self.output.iter_mut()
            {
                *vel *= self.max_velocity_ / max_velocity;
            }
        }
    }

    pub fn get_target(&self, name : &str)->f32
    {
        match self.output.get(name) {
            Some(vel)=>{
                return *vel
            }
            None=>{
                log_err!("[Swerve][get_target] この名前のユニットが存在しない");

                return 0.0
            }
        }
    }
}