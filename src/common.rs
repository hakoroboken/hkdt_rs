use rand::{self, Rng};

#[derive(Debug, Clone, Copy)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
}

impl Position2D {
    pub fn new(x_: f32, y_: f32, yaw_: f32) -> Position2D {
        return Position2D {
            x: x_,
            y: y_,
            yaw: yaw_,
        };
    }

    pub fn norm_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn random_pointcloud(
        num: usize,
        min_x: f32,
        max_x: f32,
        min_y: f32,
        max_y: f32,
    ) -> Vec<Self> {
        let mut rng = rand::thread_rng();

        let points: Vec<Self> = (0..num)
            .map(|_| {
                Position2D::new(
                    rng.gen_range(min_x..max_x),
                    rng.gen_range(min_y..max_y),
                    0.0,
                )
            })
            .collect();

        points
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
