use crate::common::Position2D;

#[derive(Debug)]
pub struct KdNode
{
    point : Position2D,
    left : Option<usize>,
    right : Option<usize>,
    axis : usize // 0 for x-axis, 1 for y-axis
}

#[derive(Debug)]
pub struct KdTree
{
    nodes : Vec<KdNode>,
    root : Option<usize>
}

impl KdTree {
    pub fn new()->Self
    {
        return KdTree { nodes: Vec::new(), root: None };
    }

    // fn build(&mut self, points : Vec<Position2D>)
    // {
    //     self.root = self.build_recursive(points, 0);
    // }
}