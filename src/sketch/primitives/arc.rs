use nalgebra::{Matrix, SMatrix};

use crate::sketch::point2::Point2;

use super::Parametric;

#[derive(Debug)]
pub struct Arc {
    data: [f64; 5],
    gradient: [f64; 5],

    clockwise: bool,
}

impl Arc {
    pub fn new(center: Point2, radius: f64, clockwise: bool, start_angle: f64, end_angle: f64) -> Self {
        Self {
            data: [center.x, center.y, radius, start_angle, end_angle],
            gradient: [0.0; 5],

            clockwise: clockwise,
        }
    }

    pub fn center(&self) -> Point2 {
        Point2 {
            x: self.data[0],
            y: self.data[1],
        }
    }

    pub fn radius(&self) -> f64 {
        self.data[2]
    }

    pub fn start_angle(&self) -> f64 {
        self.data[3]
    }

    pub fn end_angle(&self) -> f64 {
        self.data[4]
    }

    pub fn clockwise(&self) -> bool {
        self.clockwise
    }

    pub fn set_center(&mut self, center: Point2) {
        self.data[0] = center.x;
        self.data[1] = center.y;
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.data[2] = radius;
    }

    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.data[3] = start_angle;
    }

    pub fn set_end_angle(&mut self, end_angle: f64) {
        self.data[4] = end_angle;
    }

    pub fn set_clockwise(&mut self, clockwise: bool) {
        self.clockwise = clockwise;
    }

    pub fn start_point(&self) -> Point2 {
        let center = self.center();
        let radius = self.radius();
        let angle = self.start_angle();

        Point2 {
            x: center.x + radius * angle.cos(),
            y: center.y + radius * angle.sin(),
        }
    }

    pub fn end_point(&self) -> Point2 {
        let center = self.center();
        let radius = self.radius();
        let angle = self.end_angle();

        Point2 {
            x: center.x + radius * angle.cos(),
            y: center.y + radius * angle.sin(),
        }
    }

    pub fn start_point_gradient(&self) -> SMatrix<f64, 2, 5> {
        let radius = self.radius();
        let angle = self.start_angle();

        SMatrix::<f64, 2, 5>::from_row_slice(
            &[
                1.0, 0.0, angle.cos(), -radius * angle.sin(), 0.0,
                0.0, 1.0, angle.sin(), radius * angle.cos(), 0.0,
            ]
        )
    }

    pub fn end_point_gradient(&self) -> SMatrix<f64, 2, 5> {
        let radius = self.radius();
        let angle = self.end_angle();

        SMatrix::<f64, 2, 5>::from_row_slice(
            &[
                1.0, 0.0, angle.cos(), -radius * angle.sin(), 0.0,
                0.0, 1.0, angle.sin(), radius * angle.cos(), 0.0,
                // 1.0, 0.0,
                // 0.0, 1.0,
                // angle.cos(), angle.sin(),
                // -radius * angle.sin(), radius * angle.cos(),
                // 0.0, 0.0,
            ]
        )
    }

    pub fn add_to_gradient(
        &mut self,
        gradient_center_x: f64,
        gradient_center_y: f64,
        gradient_radius: f64,
        gradient_start_angle: f64,
        gradient_end_angle: f64,
    ) {
        self.gradient[0] += gradient_center_x;
        self.gradient[1] += gradient_center_y;
        self.gradient[2] += gradient_radius;
        self.gradient[3] += gradient_start_angle;
        self.gradient[4] += gradient_end_angle;
    }
}

impl Parametric for Arc {
    fn zero_gradient(&mut self) {
        self.gradient = [0.0; 5];
    }

    fn step(&mut self, step_size: f64) {
        for i in 0..5 {
            self.data[i] -= step_size * self.gradient[i];
        }
    }
}
