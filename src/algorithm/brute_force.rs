use crate::common::Position2D;

pub fn brute_force(point_cloud: &[Position2D], target: Position2D) -> (usize, f32) {
    let mut best_d = std::f32::INFINITY;
    let mut best_i = 0;

    let tx = target.x;
    let ty = target.y;

    for (i, p) in point_cloud.iter().enumerate() {
        let dx = p.x - tx;
        let dy = p.y - ty;
        let d = dx * dx + dy * dy;

        if d < best_d {
            best_d = d;
            best_i = i;
        }
    }

    (best_i, best_d)
}
