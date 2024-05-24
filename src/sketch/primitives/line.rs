use crate::sketch::point2::Point2;

use super::Parametric;

#[derive(Debug)]
pub struct Line {
    data: [f64; 4],
    gradient: [f64; 4],
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self {
            data: [start.x, start.y, end.x, end.y],
            gradient: [0.0; 4],
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

    pub fn set_start(&mut self, start: Point2) {
        self.data[0] = start.x;
        self.data[1] = start.y;
    }

    pub fn set_end(&mut self, end: Point2) {
        self.data[2] = end.x;
        self.data[3] = end.y;
    }

    pub fn add_to_gradient(
        &mut self,
        gradient_start_x: f64,
        gradient_start_y: f64,
        gradient_end_x: f64,
        gradient_end_y: f64,
    ) {
        self.gradient[0] += gradient_start_x;
        self.gradient[1] += gradient_start_y;
        self.gradient[2] += gradient_end_x;
        self.gradient[3] += gradient_end_y;
    }
}

impl Parametric for Line {
    fn zero_gradient(&mut self) {
        self.gradient = [0.0; 4];
    }

    fn step(&mut self, step_size: f64) {
        for i in 0..4 {
            self.data[i] -= step_size * self.gradient[i];
        }
    }
}
