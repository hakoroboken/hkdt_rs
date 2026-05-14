use nalgebra::Isometry2;
use rand::{self, Rng};

extern crate nalgebra;

pub type Point2 = nalgebra::Vector2<f32>;
pub fn random_pointcloud2d(
    num: usize,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
)->Vec<Point2>
{
    let mut rng = rand::thread_rng();

    let points: Vec<Point2> = (0..num)
        .map(|_| {
            Point2::new(
                rng.gen_range(min_x..max_x),
                rng.gen_range(min_y..max_y)
            )
        })
        .collect();

    points
}

#[derive(Debug, Clone, Copy)]
pub struct Position2D {
    iso : nalgebra::Isometry2<f32>
}

impl Position2D {
    pub fn new(x: f32, y: f32, yaw: f32) -> Position2D {
        return Position2D {
            iso : Isometry2::new(Point2::new(x, y), yaw)
        };
    }

    pub fn norm(&self) -> f32 {
        return self.iso.translation.vector.norm()
    }

    pub fn get_position(&self)->Point2
    {
        Point2::new(self.iso.translation.x, self.iso.translation.y)
    }

    pub fn get_theta(&self)->f32
    {
        self.iso.rotation.angle()
    }
}

// ログ出力のためのマクロ
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{}", colored::Colorize::green(format!($($arg)*).as_str()));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{}", colored::Colorize::yellow(format!($($arg)*).as_str()));
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        println!("{}", colored::Colorize::red(format!($($arg)*).as_str()));
    };
}
