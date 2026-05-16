use nalgebra::{Isometry2, Translation2};

use crate::algorithm::kd_tree::KdTree;
use crate::common::{Position2D, Vec2};
use crate::log_err;

pub fn icp(source: Vec<Vec2>, target: Vec<Vec2>, max_iterations: usize) -> Position2D {
    let kdtree = KdTree::new(target.clone());

    let mut transform = Isometry2::identity();

    for _ in 0..max_iterations {
        let mut pairs = Vec::new();
        for (_, src_p) in source.iter().enumerate() {
            let transformed_src =
                transform.transform_point(&nalgebra::Point2::new(src_p.x, src_p.y));
            let transformed_src_p = Vec2::new(transformed_src.x, transformed_src.y);
            match kdtree.nearest(&transformed_src_p) {
                Some((nearest, _dist)) => {
                    pairs.push((transformed_src_p, nearest));
                }
                None => {}
            }
        }

        let pair_num = pairs.len();
        if pair_num < 3 {
            log_err!("[ICP]ペアの数が足りません。");
            return Position2D::new(0.0, 0.0, 0.0);
        }

        let mut source_centroid = Vec2::zeros();
        let mut target_centroid = Vec2::zeros();

        for (_, (src, tar)) in pairs.iter().enumerate() {
            source_centroid += src;
            target_centroid += tar;
        }

        source_centroid /= pair_num as f32;
        target_centroid /= pair_num as f32;

        let mut s = 0.0;
        let mut c = 0.0;

        for (_, (src, tar)) in pairs.iter().enumerate() {
            let centered_src = src - source_centroid;
            let centered_tar = tar - target_centroid;

            s += centered_src.x * centered_tar.y - centered_src.y * centered_tar.x;

            c += centered_src.x * centered_tar.x + centered_src.y * centered_tar.y;
        }

        let yaw = s.atan2(c);

        let rot = nalgebra::Rotation2::new(yaw);

        let t = target_centroid - rot.matrix() * source_centroid;

        let delta_trans = Isometry2::from_parts(Translation2::from(t), rot.into());

        transform = delta_trans * transform;

        let dx = delta_trans.translation.x;
        let dy = delta_trans.translation.y;
        let dtheta = delta_trans.rotation.angle();

        if dx.abs() < 1e-6 && dy.abs() < 1e-6 && dtheta.abs() < 1e-6 {
            break;
        }
    }

    Position2D::new(
        transform.translation.x,
        transform.translation.y,
        transform.rotation.angle(),
    )
}
