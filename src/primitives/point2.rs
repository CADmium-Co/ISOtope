use nalgebra::{DVectorView, SMatrix, SMatrixView, Vector2};
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use super::{PrimitiveCell, PrimitiveLike};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Point2 {
    data: Vector2<f64>,
    gradient: Vector2<f64>,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            data: Vector2::new(x, y),
            gradient: Vector2::zeros(),
        }
    }

    pub fn x(&self) -> f64 {
        self.data.x
    }

    pub fn y(&self) -> f64 {
        self.data.y
    }

    pub fn data(&self) -> Vector2<f64> {
        self.data
    }

    pub fn point_gradient(&self) -> SMatrix<f64, 2, 2> {
        SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 0.0, 0.0, 1.0])
    }

    pub fn set_x(&mut self, x: f64) {
        assert!(x.is_finite());
        self.data.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        assert!(y.is_finite());
        self.data.y = y;
    }

    pub fn add_to_gradient(&mut self, gradient: SMatrixView<f64, 1, 2>) {
        // Panic if nan or inf is encountered
        assert!(gradient.iter().all(|x| x.is_finite()));
        self.gradient += gradient.transpose();
    }
}

impl PrimitiveLike for Point2 {
    fn references(&self) -> Vec<PrimitiveCell> {
        vec![]
    }

    fn zero_gradient(&mut self) {
        self.gradient = Vector2::zeros();
    }

    fn get_data(&self) -> DVectorView<f64> {
        self.data.as_view()
    }

    fn set_data(&mut self, data: DVectorView<f64>) {
        assert!(data.iter().all(|x| x.is_finite()));
        self.data = Vector2::from_row_slice(data.as_slice());
    }

    fn get_gradient(&self) -> DVectorView<f64> {
        self.gradient.as_view()
    }

    fn to_primitive(&self) -> super::Primitive {
        super::Primitive::Point2(self.clone())
    }
}
