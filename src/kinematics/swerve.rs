use crate::kinematics::common::UnitPosition;
use crate::log_err;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Target
{
    pub steer_angle : f32,
    pub target_velocity : f32
}
impl Target {
    pub fn new(angle : f32, vel : f32)->Target
    {
        Target { steer_angle: angle, target_velocity: vel }
    }
}

/// 独立ステアリング機構の足回り出力を計算する
/// * `swerve_position` - ロボット中心からのユニットの位置
/// * `swerve_target` - 各ユニットの目標値。
/// * `max_velocity` - 最大速度[m/s]を入力する
pub struct Swerve
{
    swerve_position_ : UnitPosition,
    swerve_target_ : HashMap<String, Target>,
    max_velocity_ : f32
}


impl Swerve {
    /// 初期化関数
    /// * `unit_data` - ロボット中心からのユニットの位置
    /// * `max_velocity` - 最大速度[m/s]を入力する
    pub fn new(unit_data : UnitPosition, max_velocity : f32)->Self
    {
        return Swerve 
        {
             swerve_position_: unit_data,
             swerve_target_ : HashMap::new(),
             max_velocity_ : max_velocity
        }
    }

    pub fn compute(&mut self, x_vec : f32, y_vec: f32, rotation_vec : f32)
    {
        self.swerve_target_.clear();
        let mut max_velocity = 0.0;

        for (pos_name, pos) in self.swerve_position_.data.iter()
        {
            let name = String::from(pos_name);

            let x_velocity = x_vec - rotation_vec * pos.y;
            let y_velocity = y_vec + rotation_vec * pos.x;

            let vel = (x_velocity*x_velocity + y_velocity*y_velocity).sqrt();
            let angle = y_velocity.atan2(x_velocity);

            let target = Target::new(angle, vel);

            if max_velocity < vel
            {
                max_velocity = vel;
            }

            self.swerve_target_.insert(name, target);
        }

        if max_velocity > self.max_velocity_
        {
            for (_name, target) in self.swerve_target_.iter_mut()
            {
                target.target_velocity *= self.max_velocity_ / max_velocity;
            }
        }

        return;
    }

    pub fn get_target(&self, name : &str)->Target
    {
        match self.swerve_target_.get(name) {
            Some(target)=>{

                return target.clone()
            }
            None=>{
                log_err!("[Swerve][get_target] この名前のユニットが存在しない");

                return Target::new(0.0, 0.0)
            }
        }
    }
}