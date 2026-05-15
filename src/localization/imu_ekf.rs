extern crate nalgebra;
use nalgebra::{Matrix3, Matrix2, Matrix3x2};

use crate::common::{Vec3, Vec2};

/// extended kalman filter for posture estimation from 9-axis imu
pub struct ImuEKF9 {
    state: Vec3,
    predict_state: Vec3,
    observe_state: Vec3,
    cov_matrix: Matrix3<f32>,
}

impl ImuEKF9 {
    /// 初期化
    pub fn new() -> Self {
        Self {
            state: Vec3::new(0.0, 0.0, 0.0),
            predict_state: Vec3::new(0.0, 0.0, 0.0),
            observe_state: Vec3::new(0.0, 0.0, 0.0),
            cov_matrix: Matrix3::identity(),
        }
    }

    pub fn compute(
        &mut self,
        angular_velocity: Vec3,
        linear_accel: Vec3,
        mag_field: Vec3,
        dt: f32,
    ) {
        self.predict(angular_velocity, dt);
        self.observe(linear_accel, mag_field);

        let predict_jacob = self.calc_jacob(angular_velocity, dt);
        let obs_jacob = Matrix3::<f32>::identity();
        let predict_noise = Matrix3::<f32>::identity() * 0.01;
        let observe_noise = Matrix3::<f32>::identity() * 0.1;

        let predict_distr = predict_jacob * self.cov_matrix * predict_jacob.transpose() + predict_noise;
        let observe_distr = obs_jacob * predict_distr * obs_jacob.transpose() + observe_noise;

        let kalman_gain = predict_distr * obs_jacob.transpose() * observe_distr.try_inverse().unwrap();

        self.state = self.predict_state + kalman_gain * (self.observe_state - self.predict_state);
        self.cov_matrix = (obs_jacob - kalman_gain) * predict_distr;
    }

    pub fn get_euler(&self)->Vec3
    {
        self.state
    }

    /// 角速度により未来の状態を予測する
    fn predict(&mut self, angular_velocity: Vec3, dt: f32) {
        let cos_x = self.state.x.cos();
        let sin_x = self.state.x.sin();

        let cos_y = self.state.y.cos();
        let tan_y = self.state.y.tan();
        // 前回の推定値から予測モデルを計算する
        let f = Matrix3::<f32>::new(
            1.0,
            tan_y * sin_x,
            tan_y * cos_x,
            0.0,
            cos_x,
            -sin_x,
            0.0,
            sin_x / cos_y,
            cos_x / cos_y,
        );

        // 入力行列を計算する
        let input_matrix = angular_velocity * dt;

        self.predict_state = self.state + f * input_matrix;
    }

    fn calc_jacob(&mut self, angular_velocity: Vec3, dt: f32) -> Matrix3<f32> {
        let omega_x = angular_velocity.x;
        let omega_y = angular_velocity.y;
        let omega_z = angular_velocity.z;

        let sinx = self.state.x.sin();
        let siny = self.state.y.sin();
        let cosx = self.state.x.cos();
        let cosy = self.state.y.cos();
        let tany = self.state.y.tan();
        
        return Matrix3::new(
            1.0+(omega_x+omega_y*tany*cosx - omega_z*tany*sinx)*dt, 
            (omega_y*(sinx/(cosy*cosy)) + omega_z*(cosx/(cosy*cosy)))*dt,
            0.0,
            (-omega_y*sinx-omega_z*cosx)*dt, 
            1.0, 
            0.0,
            (omega_y*(cosx/cosy) - omega_z*(sinx/cosy))*dt, 
            (omega_y*sinx*(siny/(cosy*cosy)) + omega_z*cosx*(siny/(cosy*cosy)))*dt, 
            1.0);
    }

    fn observe(&mut self, linear_accel: Vec3, mag_field: Vec3) {

        let x = (-linear_accel.y).atan2(-linear_accel.x);

        let y = linear_accel.x.atan2((linear_accel.y*linear_accel.y+linear_accel.z*linear_accel.z).sqrt());

        let mag_x = mag_field.x;
        let mag_y = mag_field.y;
        let mag_z = mag_field.z;
        let sinx = x.sin();
        let siny = y.sin();
        let cosx = x.cos();
        let cosy = y.cos();

        let nu = mag_x*cosy + mag_y * siny * sinx + mag_z * siny * cosx;
        let de = mag_y * cosx - mag_z * sinx;

        // let z = mag_y.atan2(mag_x);
        let z = nu.atan2(de);

        self.observe_state = Vec3::new(x,y,z);
    }
}

pub struct ImuEKF6 {
    state: Vec3,
    predict_state: Vec3,
    observe_state: Vec2,
    cov_matrix: Matrix3<f32>,
}

impl ImuEKF6 {
    /// 初期化
    pub fn new() -> Self {
        Self {
            state: Vec3::new(0.0, 0.0, 0.0),
            predict_state: Vec3::new(0.0, 0.0, 0.0),
            observe_state: Vec2::new(0.0, 0.0),
            cov_matrix: Matrix3::identity(),
        }
    }

    pub fn compute(
        &mut self,
        angular_velocity: Vec3,
        linear_accel: Vec3,
        dt: f32,
    ) {
        self.predict(angular_velocity, dt);
        self.observe(linear_accel);

        let predict_jacob = self.calc_jacob(angular_velocity, dt);
        let obs_jacob = Matrix3x2::<f32>::identity();

        let predict_noise = Matrix3::<f32>::identity() * 0.001;
        let observe_noise = Matrix2::<f32>::identity() * 0.1;

        let predict_distr = predict_jacob * self.cov_matrix * predict_jacob.transpose() + predict_noise;
        let observe_distr = obs_jacob.transpose() * predict_distr * obs_jacob + observe_noise;

        // ここのobs_jacobはobs_jacobが入るのではなく、ただ3x2単位行列がほしいだけ
        let kalman_gain = predict_distr * obs_jacob * observe_distr.try_inverse().unwrap();

        // ここのobs_jacobもそう
        self.state = self.predict_state + kalman_gain * (self.observe_state - obs_jacob.transpose() * self.predict_state);
        // ここもそう
        self.cov_matrix = (Matrix3::<f32>::identity() - kalman_gain * obs_jacob.transpose()) * predict_distr;
    }

    pub fn get_euler(&self)->Vec3
    {
        self.state
    }

    /// 角速度により未来の状態を予測する
    fn predict(&mut self, angular_velocity: Vec3, dt: f32) {
        let cos_x = self.state.x.cos();
        let sin_x = self.state.x.sin();

        let cos_y = self.state.y.cos();
        let tan_y = self.state.y.tan();
        // 前回の推定値から予測モデルを計算する
        let f = Matrix3::<f32>::new(
            1.0,
            tan_y * sin_x,
            tan_y * cos_x,
            0.0,
            cos_x,
            -sin_x,
            0.0,
            sin_x / cos_y,
            cos_x / cos_y,
        );

        // 入力行列を計算する
        let input_matrix = angular_velocity * dt;

        self.predict_state = self.state + f * input_matrix;
    }

    fn calc_jacob(&mut self, angular_velocity: Vec3, dt: f32) -> Matrix3<f32> {
        let omega_x = angular_velocity.x;
        let omega_y = angular_velocity.y;
        let omega_z = angular_velocity.z;

        let sinx = self.state.x.sin();
        let siny = self.state.y.sin();
        let cosx = self.state.x.cos();
        let cosy = self.state.y.cos();
        let tany = self.state.y.tan();
        
        return Matrix3::new(
            1.0+(omega_x+omega_y*tany*cosx - omega_z*tany*sinx)*dt, 
            (omega_y*(sinx/(cosy*cosy)) + omega_z*(cosx/(cosy*cosy)))*dt,
            0.0,
            (-omega_y*sinx-omega_z*cosx)*dt, 
            1.0, 
            0.0,
            (omega_y*(cosx/cosy) - omega_z*(sinx/cosy))*dt, 
            (omega_y*sinx*(siny/(cosy*cosy)) + omega_z*cosx*(siny/(cosy*cosy)))*dt, 
            1.0);
    }

    fn observe(&mut self, linear_accel: Vec3) {

        let x = (-linear_accel.y).atan2(-linear_accel.x);

        let y = linear_accel.x.atan2((linear_accel.y*linear_accel.y+linear_accel.z*linear_accel.z).sqrt());

        self.observe_state = Vec2::new(x,y);
    }
}
