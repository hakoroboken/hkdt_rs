//! ３輪オムニ制御の例
//! 位置イメージ
//!
//! front_left     front_right
//!
//!          center(ロボットの中央)
//!
//!           rear

use hkdt_rs::{
    common::Position2D,               //２次元座標を表現する構造体
    kinematics::common::UnitPosition, // 各ユニットの位置を管理する構造体
    kinematics::omni::Omni,           // オムニの制御を行う構造体
};

use hkdt_rs::log_info; // ログ出力のためのマクロ

fn main() {
    log_info!("３輪オムニ制御の例");

    //まずは各ユニットのロボット中心からの位置を定義する
    let mut unit_data = UnitPosition::new();

    // Position2Dはx座標、y座標、角度を表す構造体です。

    // 左前のユニット
    let front_left_pos = Position2D::new(-0.6, 0.8, 45_f32.to_radians());
    unit_data.add_unit("front_left", front_left_pos);

    // 右前のユニット
    let front_right_pos = Position2D::new(0.6, 0.8, 315_f32.to_radians());
    unit_data.add_unit("front_right", front_right_pos);

    // 後のユニット
    let rear_left_pos = Position2D::new(0.0, -1.0, 0.0);
    unit_data.add_unit("rear", rear_left_pos);

    // OmniをUnitPositionを使って初期化します
    // ここではロボットの最大速度を1.0m/sに設定しています。必要に応じてこの値を変更してください。
    //　また、内部で変数の上書きが発生するので必ずmutをつけてください。
    let mut omni = Omni::new(unit_data, 1.0);

    // 1.0[m/s]で前進する例
    // compute関数は左から順番に、x方向の速度、y方向の速度、回転速度を指定します。
    omni.compute(0.0, 1.0, 0.0);

    // 計算された各ユニットの速度とステアリング角度をログに出力します
    // 左前ユニットの目標速度を取得
    // get_target関数はadd_unitで設定したユニット名を引数にとり、そのユニットの目標速度を返します。
    // またこのtargetは[m/s]で表されているので、必要に応じて単位変換してください。

    let front_left_target = omni.get_target("front_left");
    log_info!("Front Left 目標速度: {}[m/s]", front_left_target);

    let front_right_target = omni.get_target("front_right");
    log_info!("Front Right 目標速度: {}[m/s]", front_right_target);

    let rear_target = omni.get_target("rear");
    log_info!("Rear Left 目標速度: {}[m/s]", rear_target);
}
