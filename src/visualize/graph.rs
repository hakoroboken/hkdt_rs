use crate::common::Position2D;

use plotlib::{page::Page, repr::Plot, style::*, view::ContinuousView};

pub enum PlotColor {
    Red,
    Green,
    Blue,
    Brack,
}

impl PlotColor {
    fn to_str(&self) -> &str {
        match self {
            PlotColor::Red => RED,
            PlotColor::Blue => BLUE,
            PlotColor::Brack => BRACK,
            PlotColor::Green => GREEN,
        }
    }
}

const RED: &str = "#ff0202";
const GREEN: &str = "#24ff02";
const BLUE: &str = "#0228ff";
const BRACK: &str = "#000000";

pub struct Plotter {
    min_x: f64,
    max_x: f64,
    delta_x: f64,
    y_size: f64,

    plots: Vec<Plot>,
}

impl Plotter {
    pub fn new(min_x_: f64, max_x: f64, delta_x_: f64, y_size_: f64) -> Self {
        Self {
            min_x: min_x_,
            max_x: max_x,
            delta_x: delta_x_,
            y_size: y_size_,
            plots: Vec::new(),
        }
    }

    pub fn add_func<F: Fn(f64) -> f64>(&mut self, f: F, color: PlotColor) {
        let mut x = self.min_x;
        let mut result = Vec::<(f64, f64)>::new();

        while x < self.max_x {
            result.push((x, f(x)));

            x += self.delta_x;
        }

        let new_plot =
            Plot::new(result).point_style(PointStyle::new().colour(color.to_str()).size(1.0));

        self.plots.push(new_plot);
    }

    pub fn add_points(&mut self, points: Vec<Position2D>, color: PlotColor) {
        let mut pt_vec = vec![];

        for p in points {
            pt_vec.push((p.x as f64, p.y as f64));
        }

        let new_plot =
            Plot::new(pt_vec).point_style(PointStyle::new().colour(color.to_str()).size(3.0));

        self.plots.push(new_plot);
    }

    pub fn add_point(&mut self, point: Position2D, color: PlotColor) {
        let pt_vec = vec![(point.x as f64, point.y as f64)];

        let new_plot =
            Plot::new(pt_vec).point_style(PointStyle::new().colour(color.to_str()).size(3.0));
        self.plots.push(new_plot);
    }

    pub fn save(&self, file_name: &str) {
        let view = ContinuousView::new();
        let mut arranged_view = view
            .x_range(self.min_x, self.max_x)
            .y_range(-1.0 * self.y_size, self.y_size)
            .x_label("x")
            .y_label("y")
            .x_max_ticks((1.0 / self.delta_x) as usize)
            .y_max_ticks((1.0 / self.delta_x) as usize);

        for i in 0..self.plots.len() {
            arranged_view = arranged_view.add(self.plots[i].clone());
        }

        Page::single(&arranged_view).save(file_name).unwrap();
    }
}
