use std::{cell::RefCell, rc::Rc};

use nalgebra::{DVector, DVectorView};

pub mod arc;
pub mod circle;
pub mod line;
pub mod point2;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a fixed number of parameters that can be used for gradient descent.
pub trait Parametric {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>>;
    fn zero_gradient(&mut self);
    fn step(&mut self, step_size: f64);
    fn get_data(&self) -> DVector<f64>;
    fn set_data(&mut self, data: DVectorView<f64>);
    fn get_gradient(&self) -> DVector<f64>;
}
