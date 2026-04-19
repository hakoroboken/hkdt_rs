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

    pub fn squared_distance(&self) -> f32 {
        return self.x * self.x + self.y * self.y;
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
