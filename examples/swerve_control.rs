//! 独立ステアリング制御の例
//! ここでは４輪独立ステアリングの例を示します。各ユニットの位置を定義し、制御する方法を説明します。
//! ステアリング位置イメージ
//! front_left     front_right
//!
//!          center(ロボットの中央)
//!
//! rear_left      rear_right

use hkdt_rs::{
    common::Position2D,               //２次元座標を表現する構造体
    kinematics::common::UnitPosition, // 各ユニットの位置を管理する構造体
    kinematics::swerve::Swerve,       // 独立ステアリングの制御を行う構造体
    kinematics::swerve::Target,       // 各ユニットの目標速度と目標角度を表す構造体
};

use hkdt_rs::log_info; // ログ出力のためのマクロ

fn main() {
    log_info!("４輪独立ステアリング制御の例");

    //まずは各ユニットのロボット中心からの位置を定義する
    let mut unit_data = UnitPosition::new();

    // Position2Dはx座標、y座標、角度を表す構造体でステアリング位置を指定するときには角度は使わないので０にしてます。
    // ここではあくまでステアリング位置を２次元座標で入力することを想定しているため、角度は無視されます。

    // 左前のユニット
    let front_left_pos = Position2D::new(-0.25, 0.25, 0.0);
    unit_data.add_unit("front_left", front_left_pos);

    // 右前のユニット
    let front_right_pos = Position2D::new(0.25, 0.25, 0.0);
    unit_data.add_unit("front_right", front_right_pos);

    // 左後のユニット
    let rear_left_pos = Position2D::new(-0.25, -0.25, 0.0);
    unit_data.add_unit("rear_left", rear_left_pos);

    // 右後のユニット
    let rear_right_pos = Position2D::new(0.25, -0.25, 0.0);
    unit_data.add_unit("rear_right", rear_right_pos);

    // SwerveをUnitPositionを使って初期化します
    // ここではロボットの最大速度を2.6m/sに設定しています。必要に応じてこの値を変更してください。
    //　また、内部で変数の上書きが発生するので必ずmutをつけてください。
    let mut swerve = Swerve::new(unit_data, 2.6);

    // 1.0[m/s]で前進する例
    // compute関数は左から順番に、x方向の速度、y方向の速度、回転速度を指定します。
    swerve.compute(0.0, 1.0, 0.0);

    // 計算された各ユニットの速度とステアリング角度をログに出力します
    // 左前ユニットの目標速度と目標角度を取得
    // get_target関数はadd_unitで設定したユニット名を引数にとり、そのユニットの目標速度と目標角度を返します。
    // またこのTarget構造体のtarget_velocityは[m/s]、steer_angleは[rad]で表されているので、必要に応じて単位変換してください。
    let front_left_target: Target = swerve.get_target("front_left");
    // ここでは表示するときにわかりやすいようにto_degrees関数を使ってラジアンを度に変換しています。
    log_info!(
        "Front Left 目標速度: {}[m/s], 目標角度: {}[deg]",
        front_left_target.target_velocity,
        front_left_target.steer_angle.to_degrees()
    );

    let front_right_target: Target = swerve.get_target("front_right");
    log_info!(
        "Front Right 目標速度: {}[m/s], 目標角度: {}[deg]",
        front_right_target.target_velocity,
        front_right_target.steer_angle.to_degrees()
    );

    let rear_left_target: Target = swerve.get_target("rear_left");
    log_info!(
        "Rear Left 目標速度: {}[m/s], 目標角度: {}[deg]",
        rear_left_target.target_velocity,
        rear_left_target.steer_angle.to_degrees()
    );

    let rear_right_target: Target = swerve.get_target("rear_right");
    log_info!(
        "Rear Right 目標速度: {}[m/s], 目標角度: {}[deg]",
        rear_right_target.target_velocity,
        rear_right_target.steer_angle.to_degrees()
    );
}
