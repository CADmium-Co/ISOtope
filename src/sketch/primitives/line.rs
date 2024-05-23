use crate::sketch::point2::Point2;

use super::Parametric;

pub struct Line {
    data: [f64; 4],
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self {
            data: [start.x, start.y, end.x, end.y],
        }
    }

    pub fn start(&self) -> Point2 {
        Point2 {
            x: self.data[0],
            y: self.data[1],
        }
    }

    pub fn end(&self) -> Point2 {
        Point2 {
            x: self.data[2],
            y: self.data[3],
        }
    }

    pub fn gradient_to_data(gradient_start_x: f64, gradient_start_y: f64, gradient_end_x: f64, gradient_end_y: f64) -> [f64; 4] {
        [gradient_start_x, gradient_start_y, gradient_end_x, gradient_end_y]
    }
}

impl Parametric<4> for Line {
    fn from_data(data: [f64; 4]) -> Self {
        Self {
            data,
        }
    }

    fn data(&self) -> [f64; 4] {
        self.data
    }
}

