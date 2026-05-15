use crate::common::{Vec2, Position2D};
use crate::algorithm::kd_tree::KdTree;
use crate::log_err;

pub fn icp(
    source : Vec<Vec2>,
    target : Vec<Vec2>,
    max_iterations : usize,
)-> Position2D
{
    let kdtree = KdTree::new(target.clone());

    let mut transformed = source.clone();

    let mut transform = Position2D::new(0.0, 0.0, 0.0);

    for _ in 0..max_iterations
    {
        let mut pairs = Vec::new();
        for (_, src_p) in transformed.iter().enumerate()
        {
            match kdtree.nearest(&src_p)
            {
                Some((nearest, _dist))=>{
                    pairs.push((*src_p, nearest));
                }
                None=>{

                }
            }
        }

        let pair_num = pairs.len();
        if pair_num < 3
        {
            log_err!("[ICP]ペアの数が足りません。");
            return Position2D::new(0.0, 0.0, 0.0)
        }

        // let mut 
    }    

    transform
}