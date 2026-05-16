//　２つの点群の間の移動と回転を推定する例
// 点と点を対応させるIterativeClosestPointアルゴリズムを使っている
// 内部ではKdTreeを用いた最近傍探索が使われている
// 30度くらいならしっかり求まるが90度とか変換すると局所解にはまったりする。
// なんなら30度くらいでも局所解にはまったりする
// だから点と点を対応させるIterativeClosestPointは回転に弱い

use hkdt_rs::{
    common::{Position2D,random_pointcloud2d, transform_pointcloud2d},
    localization::p2p_icp::icp, // 点と点対応なのでp2pという名前つけてる
    log_info
};

fn main()
{
    let point_cloud1 = random_pointcloud2d(400, -12.0..12.0, -12.0..12.0);

    // xに8[m]とyに6[m]進めて30度回転させる変換
    let trans = Position2D::new(8.0, 6.0, 30.0_f32.to_radians());

    // point_cloud1をtransにより移動させた点群を容易
    let point_cloud2 = transform_pointcloud2d(point_cloud1.clone(), trans);

    let timer = std::time::Instant::now();
    // point_cloud1からpoint_cloud2へどのくらい移動しているかを推定する
    // 結果がpへ入る
    let result = icp(point_cloud1, point_cloud2, 10000);

    log_info!("===== ICPの結果 =====");
    log_info!("経過時間=\t{}[us]", timer.elapsed().as_micros());

    let position = result.get_position();
    log_info!("x方向変位=\t{}[m]", position.x);
    log_info!("y方向変位=\t{}[m]", position.y);
    // ラジアンで帰ってくるので度に変換して表示
    log_info!("回転=\t\t{}[deg]", result.get_theta().to_degrees());
}