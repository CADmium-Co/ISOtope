use std::{cell::RefCell, rc::Rc};

use nalgebra::{SMatrix, SMatrixView, SVector};

use super::{point2::Point2, Parametric};

pub struct Circle {
    center: Rc<RefCell<Point2>>,
    data: SVector<f64, 1>,
    gradient: SVector<f64, 1>,
}

impl Circle {
    pub fn new(center: Rc<RefCell<Point2>>, radius: f64) -> Self {
        Self {
            center: center,
            data: SVector::<f64, 1>::from_row_slice(&[radius]),
            gradient: SVector::<f64, 1>::zeros(),
        }
    }

    pub fn center(&self) -> Rc<RefCell<Point2>> {
        self.center.clone()
    }

    pub fn set_center(&mut self, center: Rc<RefCell<Point2>>) {
        self.center = center;
    }

    pub fn center_gradient(&self) -> SMatrix<f64, 2, 3> {
        SMatrix::<f64, 2, 3>::from_row_slice(
            &[
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
            ]
        )
    }

    pub fn radius(&self) -> f64 {
        self.data[0]
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.data[0] = radius;
    }

    pub fn radius_gradient(&self) -> SMatrix<f64, 1, 3> {
        SMatrix::<f64, 1, 3>::from_row_slice(
            &[
                0.0, 0.0, 1.0,
            ]
        )
    }

    pub fn add_to_gradient(
        &mut self,
        gradient: SMatrixView<f64, 1, 3>,
    ) {
        self.center.borrow_mut().add_to_gradient(gradient.fixed_view::<1, 2>(0, 0));
        self.gradient += gradient.fixed_view::<1, 1>(0, 2).transpose();
    }
}

impl Parametric for Circle {
    fn references(&self) -> Vec<std::rc::Rc<std::cell::RefCell<dyn Parametric>>> {
        vec![self.center.clone()]
    }

    fn zero_gradient(&mut self) {
        self.gradient = SVector::<f64, 1>::zeros();
    }

    fn step(&mut self, step_size: f64) {
        self.data -= step_size * self.gradient;
    }
}
