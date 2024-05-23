use crate::sketch::point2::Point2;

use super::Parametric;

pub struct Line<'a> {
    data: &'a mut [f64; 4],
    gradient: &'a mut [f64; 4],
}

impl<'a> Line<'a> {
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

    pub fn set_start(&mut self, start: Point2) {
        self.data[0] = start.x;
        self.data[1] = start.y;
    }

    pub fn set_end(&mut self, end: Point2) {
        self.data[2] = end.x;
        self.data[3] = end.y;
    }

    pub fn gradient_to_data(gradient_start_x: f64, gradient_start_y: f64, gradient_end_x: f64, gradient_end_y: f64) -> [f64; 4] {
        [gradient_start_x, gradient_start_y, gradient_end_x, gradient_end_y]
    }
}

impl<'a> Parametric<'a, 4> for Line<'a> {
    fn initialize(data: &'a mut [f64; 4], gradient: &'a mut [f64; 4]) -> Self {
        Self {
            data,
            gradient,
        }
    }
}

