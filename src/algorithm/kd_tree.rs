use crate::common::Position2D;

#[derive(Debug)]
pub struct KdNode {
    point: Position2D,
    left: Option<usize>,
    right: Option<usize>,
    axis: usize, // 0 for x-axis, 1 for y-axis
}

#[derive(Debug)]
pub struct KdTree {
    nodes: Vec<KdNode>,
    root: Option<usize>,
}

impl KdTree {
    pub fn new(mut points: Vec<Position2D>) -> Self {
        let mut nodes = Vec::with_capacity(points.len());
        let root = Self::build(&mut points, &mut nodes, 0);
        return KdTree {
            nodes: nodes,
            root: root,
        };
    }

    fn build(points: &mut [Position2D], nodes: &mut Vec<KdNode>, depth: usize) -> Option<usize> {
        if points.is_empty() {
            return None;
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
            left: None,
            right: None,
            axis,
        });

        // 左右の部分を再帰的にビルド
        let left_child = Self::build(left, nodes, depth + 1);
        let right_child = Self::build(right, nodes, depth + 1);

        // 現在のノードに左右の子ノードのインデックスを設定
        nodes[node_index].left = left_child;
        nodes[node_index].right = right_child;

        // 現在のノードのインデックスを返す
        return Some(node_index);
    }

    fn nearest_rec(
        &self,
        node_index: Option<usize>,
        target: &Position2D,
        best: &mut Option<(Position2D, f32)>,
    ) {
        let Some(index) = node_index else {
            return;
        };

        let node = &self.nodes[index];

        // 現在のノードとターゲットの距離を計算
        let dist = Self::distance_squared(&node.point, target);
    }

    fn distance_squared(a: &Position2D, b: &Position2D) -> f32 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        return dx * dx + dy * dy;
    }
}
