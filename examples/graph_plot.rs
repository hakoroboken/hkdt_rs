//! 点群をsvg形式で生成する例

use hkdt_rs::{
    visualize::graph::{PlotColor, Plotter},
    common::random_pointcloud2d
};

fn main()
{
    // -3.0 < x < 3.0
    // -3.0 < y < 3.0
    // 数は100個の点群を生成
    let point_cloud = random_pointcloud2d(100, -3.0..3.0, -3.0..3.0);

    // -4.0(第一引数) < x < 4.0(第二引数)
    // -4.0 < y < 4.0(第四引数)
    // グラフの分解能は0.01(第三引数)
    let mut plotter = Plotter::new(-4.0, 4.0, 0.1, 4.0);

    plotter.add_points(point_cloud, PlotColor::Blue);

    plotter.save("point_cloud.svg");
}