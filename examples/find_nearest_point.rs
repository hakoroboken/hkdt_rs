use hkdt_rs::{
    algorithm::{brute_force, kd_tree},
    common::Position2D,
    log_err, log_info,
};

// 点群の数が500ならbrute forceのほうが早くて5000あたりからkd treeのほうが早いぞ
const POINT_NUM: usize = 5000;

fn main() {
    // ランダムな点群を生成
    // -6.0 < x < 6.0
    // -6.0 < y < 6.0
    // 点の数はPOINT_NUM個
    let point_cloud = Position2D::random_pointcloud(POINT_NUM, -12.0, 12.0, -12.0, 12.0);

    // ここではx = 1.5 y = 0.0の点に対して最も距離の近い点を見つける
    let target_point = Position2D::new(1.5, 0.0, 0.0);

    kd_tree_example(point_cloud.clone(), target_point);
    brute_force_example(point_cloud.clone(), target_point);
}

fn kd_tree_example(point_cloud: Vec<Position2D>, target: Position2D) {
    let kdtree = kd_tree::KdTree::new(point_cloud.clone());

    let timer = std::time::Instant::now();

    let mut best_p = None;
    let mut best_dist = None;
    // kdtreeによる最近傍探索。
    // 見つかった場合はSomeの処理、見つからない場合はNoneの処理
    match kdtree.nearest(&target) {
        Some((nearest_point, distance)) => {
            best_p = Some(nearest_point);
            best_dist = Some(distance.sqrt());
        }
        None => {
            log_err!("最近傍探索失敗");
        }
    }
    let elapsed = timer.elapsed().as_nanos();

    log_info!("===== KdTreeの結果 =====");
    log_info!("探索にかかった時間：\t{}[ns]", elapsed);
    log_info!(
        "最近傍点座標：\tx={}, y={}",
        best_p.unwrap().x,
        best_p.unwrap().y
    );
    log_info!("目標との距離：\t{}[m]", best_dist.unwrap());
}

fn brute_force_example(point_cloud: Vec<Position2D>, target: Position2D) {
    let timer = std::time::Instant::now();

    let (best_i, best_distance_squared) = brute_force::brute_force(&point_cloud, target);

    let elapsed = timer.elapsed().as_nanos();

    log_info!("===== BruteForceの結果 =====");
    log_info!("探索にかかった時間：\t{}[ns]", elapsed);
    log_info!(
        "最近傍点座標：\tx={}, y={}",
        point_cloud[best_i].x,
        point_cloud[best_i].y
    );
    log_info!("目標との距離：\t{}[m]", best_distance_squared.sqrt());
}
