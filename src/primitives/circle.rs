use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::{DVector, DVectorView, SMatrix, SMatrixView, SVector};
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use super::{point2::Point2, PrimitiveLike, PrimitiveCell};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Circle {
    center: Rc<RefCell<Point2>>,
    data: SVector<f64, 1>,
    gradient: SVector<f64, 1>,
}

impl Circle {
    pub fn new(center: Rc<RefCell<Point2>>, radius: f64) -> Self {
        Self {
            center,
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
        SMatrix::<f64, 2, 3>::from_row_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    }

    pub fn radius(&self) -> f64 {
        self.data[0]
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.data[0] = radius;
    }

    pub fn radius_gradient(&self) -> SMatrix<f64, 1, 3> {
        SMatrix::<f64, 1, 3>::from_row_slice(&[0.0, 0.0, 1.0])
    }

    pub fn add_to_gradient(&mut self, gradient: SMatrixView<f64, 1, 3>) {
        // Panic if nan or inf is encountered
        assert!(gradient.iter().all(|x| x.is_finite()));
        self.center
            .borrow_mut()
            .add_to_gradient(gradient.fixed_view::<1, 2>(0, 0));
        self.gradient += gradient.fixed_view::<1, 1>(0, 2).transpose();
    }
}

impl PrimitiveLike for Circle {
    fn references(&self) -> Vec<PrimitiveCell> {
        vec![PrimitiveCell::Point2(self.center.clone())]
    }

    fn zero_gradient(&mut self) {
        self.gradient = SVector::<f64, 1>::zeros();
    }

    fn get_data(&self) -> DVector<f64> {
        DVector::from_row_slice(self.data.as_slice())
    }

    fn set_data(&mut self, data: DVectorView<f64>) {
        self.data.copy_from(&data);
    }

    fn get_gradient(&self) -> DVector<f64> {
        DVector::from_row_slice(self.gradient.as_slice())
    }

    fn to_primitive(&self) -> super::Primitive {
        super::Primitive::Circle(self.clone())
    }
}
