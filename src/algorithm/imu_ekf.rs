extern crate nalgebra;
pub use nalgebra::{Matrix3, Vector3};

/// extended kalman filter for posture estimation from 9-axis imu
pub struct ImuEKF9 {
    state: Vector3<f32>,
    predict_state: Vector3<f32>,
    cov_matrix: Matrix3<f32>,
}

impl ImuEKF9 {
    /// 初期化
    pub fn new() -> Self {
        Self {
            state: Vector3::new(0.0, 0.0, 0.0),
            predict_state: Vector3::new(0.0, 0.0, 0.0),
            cov_matrix: Matrix3::identity(),
        }
    }

    pub fn compute(
        &mut self,
        angular_velocity: Vector3<f32>,
        linear_accel: Vector3<f32>,
        mag_field: Vector3<f32>,
        dt: f32,
    ) {
        self.predict(angular_velocity, dt);
        let jacob = self.calc_jacob(angular_velocity, dt);
        self.update(linear_accel, mag_field);
    }

    /// 角速度により未来の状態を予測する
    fn predict(&mut self, angular_velocity: Vector3<f32>, dt: f32) {
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

    fn calc_jacob(&mut self, angular_velocity: Vector3<f32>, dt: f32) -> Matrix3<f32> {
        return Matrix3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    }

    fn update(&mut self, linear_accel: Vector3<f32>, mag_field: Vector3<f32>) {}
}
