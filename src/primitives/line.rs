use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{DVector, DVectorView, SMatrix, SMatrixView};
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use super::point2::Point2;
use super::{Parametric, ParametricCell};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Line {
    start: Rc<RefCell<Point2>>,
    end: Rc<RefCell<Point2>>,
}

impl Line {
    pub fn new(start: Rc<RefCell<Point2>>, end: Rc<RefCell<Point2>>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Rc<RefCell<Point2>> {
        self.start.clone()
    }

    pub fn set_start(&mut self, start: Rc<RefCell<Point2>>) {
        self.start = start;
    }

    pub fn start_gradient(&self) -> SMatrix<f64, 2, 4> {
        SMatrix::<f64, 2, 4>::from_row_slice(&[1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    pub fn end(&self) -> Rc<RefCell<Point2>> {
        self.end.clone()
    }

    pub fn set_end(&mut self, end: Rc<RefCell<Point2>>) {
        self.end = end;
    }

    pub fn end_gradient(&self) -> SMatrix<f64, 2, 4> {
        SMatrix::<f64, 2, 4>::from_row_slice(&[0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0])
    }

    pub fn add_to_gradient(&mut self, gradient: SMatrixView<f64, 1, 4>) {
        // Panic if nan or inf is encountered
        assert!(gradient.iter().all(|x| x.is_finite()));
        self.start
            .borrow_mut()
            .add_to_gradient(gradient.fixed_view::<1, 2>(0, 0));
        self.end
            .borrow_mut()
            .add_to_gradient(gradient.fixed_view::<1, 2>(0, 2));
    }
}

impl Parametric for Line {
    fn references(&self) -> Vec<ParametricCell> {
        vec![
            ParametricCell::Point2(self.start.clone()),
            ParametricCell::Point2(self.end.clone()),
        ]
    }

    fn zero_gradient(&mut self) {
        // Referenced points will zero their gradients automatically as they are part of the sketch
    }

    fn get_data(&self) -> DVector<f64> {
        // empty vector
        DVector::from_row_slice(&[])
    }

    fn set_data(&mut self, _data: DVectorView<f64>) {
        // Do nothing
    }

    fn get_gradient(&self) -> DVector<f64> {
        // empty vector
        DVector::from_row_slice(&[])
    }

    fn to_primitive(&self) -> super::Primitive {
        super::Primitive::Line(self.clone())
    }
}
