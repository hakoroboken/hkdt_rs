use hkdt_rs::{
    algorithm::kd_tree,
    visualize::graph::{PlotColor, Plotter},
    common::Position2D,
    log_info, log_err
};

fn main()
{
    let point_cloud = Position2D::random_pointcloud(100, -3.0, 3.0, -3.0, 3.0);

    let target_point = Position2D::new(1.5, 0.0, 0.0);

    let kdtree = kd_tree::KdTree::new(point_cloud.clone());

    let mut plotter = Plotter::new(-4.0, 4.0, 0.1, 4.0);
    plotter.add_points(point_cloud.clone(), PlotColor::Brack);

    match kdtree.nearest(&target_point)
    {
        Some((nearest_point, distance))=>{
            log_info!("最近傍探索に成功しました。 距離 = {}[m]", distance);
            log_info!("x={}, y={}", nearest_point.x, nearest_point.y);

            plotter.add_point(target_point, PlotColor::Red);
            plotter.add_point(nearest_point, PlotColor::Green);
        },
        None=>{
            log_err!("最近傍探索失敗");
        }
    }

    plotter.save("result.svg");
}