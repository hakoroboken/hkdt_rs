use crate::common::Vec2;

#[derive(Debug)]
pub struct KdNode {
    point: Vec2,
    left: usize,
    right: usize,
    axis: usize, // 0 for x-axis, 1 for y-axis
}

#[derive(Debug)]
pub struct KdTree {
    nodes: Vec<KdNode>,
    root: usize,
}

const NONE : usize = std::usize::MAX;

impl KdTree {
    pub fn new(mut points: Vec<Vec2>) -> Self {
        let mut nodes = Vec::with_capacity(points.len());
        let root = Self::build(&mut points, &mut nodes, 0);
        return KdTree {
            nodes: nodes,
            root: root,
        };
    }

    fn build(points: &mut [Vec2], nodes: &mut Vec<KdNode>, depth: usize) -> usize {
        if points.is_empty() {
            return NONE;
        }

        let axis = depth % 2;
        let mid = points.len() / 2;

        // pointsをaxisに基づいてmidで並び替える
        points.select_nth_unstable_by(mid, |a, b| {
            if axis == 0 {
                a.x.total_cmp(&b.x)
            } else {
                a.y.total_cmp(&b.y)
            }
        });

        // midの点を中央にして左右に分割
        let (left, right) = points.split_at_mut(mid);
        // 分割された右側を先頭とそれ以外に分割
        let (median, right) = right.split_first_mut().unwrap();

        // 新しいノードを作成してnodesに追加
        let node_index = nodes.len();
        nodes.push(KdNode {
            point: *median,
            left: NONE,
            right: NONE,
            axis,
        });

        // 左右の部分を再帰的にビルド
        let left_child = Self::build(left, nodes, depth + 1);
        let right_child = Self::build(right, nodes, depth + 1);

        // 現在のノードに左右の子ノードのインデックスを設定
        nodes[node_index].left = left_child;
        nodes[node_index].right = right_child;

        // 現在のノードのインデックスを返す
        return node_index;
    }

    pub fn nearest(&self, target: &Vec2) -> Option<(Vec2, f32)> {
        let mut best = None;

        self.nearest_rec(self.root, target, &mut best);

        best
    }

    fn nearest_rec(
        &self,
        node_index: usize,
        target: &Vec2,
        best: &mut Option<(Vec2, f32)>,
    ) {
        if node_index == NONE
        {
            return;
        }

        let node = &self.nodes[node_index];

        // 現在のノードとターゲットの距離を計算
        let dx = node.point.x - target.x;
        let dy = node.point.y - target.y;

        let dist = dx * dx + dy * dy;

        if best.is_none() || dist < best.unwrap().1 {
            *best = Some((node.point, dist))
        }

        let axis = node.axis;

        let (next, other, diff) = if axis == 0 {
            let diff = target.x - node.point.x;
            if diff < 0.0 {
                (node.left, node.right, diff)
            } else {
                (node.right, node.left, diff)
            }
        } else {
            let diff = target.y - node.point.y;
            if diff < 0.0 {
                (node.left, node.right, diff)
            } else {
                (node.right, node.left, diff)
            }
        };

        self.nearest_rec(next, target, best);

        if best.is_none() || diff * diff < best.unwrap().1 {
            self.nearest_rec(other, target, best);
        }
    }
}
