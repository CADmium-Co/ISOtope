use std::{cell::RefCell, rc::Rc};

use nalgebra::{SMatrix, SMatrixView, SVector, Vector2};

use super::{point2::Point2, Parametric};

#[derive(Debug)]
pub struct Arc {
    center: Rc<RefCell<Point2>>,
    data: SVector<f64, 3>,
    gradient: SVector<f64, 3>,

    clockwise: bool,
}

impl Arc {
    pub fn new(
        center: Rc<RefCell<Point2>>,
        radius: f64,
        clockwise: bool,
        start_angle: f64,
        end_angle: f64,
    ) -> Self {
        Self {
            center,
            data: SVector::<f64, 3>::from_row_slice(&[radius, start_angle, end_angle]),
            gradient: SVector::<f64, 3>::zeros(),

            clockwise,
        }
    }

    pub fn center(&self) -> Rc<RefCell<Point2>> {
        self.center.clone()
    }

    pub fn set_center(&mut self, center: Rc<RefCell<Point2>>) {
        self.center = center;
    }

    pub fn center_gradient(&self) -> SMatrix<f64, 2, 5> {
        SMatrix::<f64, 2, 5>::from_row_slice(&[1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0])
    }

    pub fn radius(&self) -> f64 {
        self.data[0]
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.data[0] = radius;
    }

    pub fn radius_gradient(&self) -> SMatrix<f64, 1, 5> {
        SMatrix::<f64, 1, 5>::from_row_slice(&[0.0, 0.0, 1.0, 0.0, 0.0])
    }

    pub fn start_angle(&self) -> f64 {
        self.data[1]
    }

    pub fn set_start_angle(&mut self, start_angle: f64) {
        self.data[1] = start_angle;
    }

    pub fn start_angle_gradient(&self) -> SMatrix<f64, 1, 5> {
        SMatrix::<f64, 1, 5>::from_row_slice(&[0.0, 0.0, 0.0, 1.0, 0.0])
    }

    pub fn end_angle(&self) -> f64 {
        self.data[2]
    }

    pub fn set_end_angle(&mut self, end_angle: f64) {
        self.data[2] = end_angle;
    }

    pub fn end_angle_gradient(&self) -> SMatrix<f64, 1, 5> {
        SMatrix::<f64, 1, 5>::from_row_slice(&[0.0, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn clockwise(&self) -> bool {
        self.clockwise
    }

    pub fn set_clockwise(&mut self, clockwise: bool) {
        self.clockwise = clockwise;
    }

    pub fn start_point(&self) -> Vector2<f64> {
        let center = self.center();
        let radius = self.radius();
        let angle = self.start_angle();
        let x = center.borrow().x() + radius * angle.cos();
        let y = center.borrow().y() + radius * angle.sin();

        Vector2::new(x, y)
    }

    pub fn start_point_gradient(&self) -> SMatrix<f64, 2, 5> {
        let radius = self.radius();
        let angle = self.start_angle();

        SMatrix::<f64, 2, 5>::from_row_slice(&[
            1.0,
            0.0,
            angle.cos(),
            -radius * angle.sin(),
            0.0,
            0.0,
            1.0,
            angle.sin(),
            radius * angle.cos(),
            0.0,
        ])
    }

    pub fn end_point(&self) -> Vector2<f64> {
        let center = self.center();
        let radius = self.radius();
        let angle = self.end_angle();
        let x = center.borrow().x() + radius * angle.cos();
        let y = center.borrow().y() + radius * angle.sin();
        Vector2::new(x, y)
    }

    pub fn end_point_gradient(&self) -> SMatrix<f64, 2, 5> {
        let radius = self.radius();
        let angle = self.end_angle();

        SMatrix::<f64, 2, 5>::from_row_slice(&[
            1.0,
            0.0,
            angle.cos(),
            0.0,
            -radius * angle.sin(),
            0.0,
            1.0,
            angle.sin(),
            0.0,
            radius * angle.cos(),
        ])
    }

    pub fn add_to_gradient(&mut self, gradient: SMatrixView<f64, 1, 5>) {
        // Panic if nan or inf is encountered
        assert!(gradient.iter().all(|x| x.is_finite()));
        self.center
            .borrow_mut()
            .add_to_gradient(gradient.fixed_view::<1, 2>(0, 0));
        self.gradient += gradient.fixed_view::<1, 3>(0, 2).transpose();
    }
}

impl Parametric for Arc {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>> {
        vec![self.center.clone()]
    }

    fn zero_gradient(&mut self) {
        self.gradient = SVector::<f64, 3>::zeros();
    }

    fn step(&mut self, step_size: f64) {
        self.data -= step_size * self.gradient;
    }
}
