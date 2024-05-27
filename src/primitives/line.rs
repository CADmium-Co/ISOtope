use std::{cell::RefCell, rc::Rc};

use nalgebra::{DVector, DVectorView, SMatrix, SMatrixView};

use super::{point2::Point2, Parametric};

#[derive(Debug, Clone)]
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
    /// Get the line's start point coordinates
    pub fn start_pt(&self) -> nalgebra::Point2<f64> {
        self.start.borrow().data().into()
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
    /// Get the line's end point coordinates
    pub fn end_pt(&self) -> nalgebra::Point2<f64> {
        self.end.borrow().data().into()
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
    fn references(&self) -> Vec<std::rc::Rc<std::cell::RefCell<dyn Parametric>>> {
        vec![self.start.clone(), self.end.clone()]
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
