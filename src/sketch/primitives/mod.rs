pub mod arc;
pub mod circle;
pub mod line;
pub mod point2;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a fixed number of parameters that can be used for gradient descent.
pub trait Parametric {
    fn zero_gradient(&mut self);
    fn step(&mut self, step_size: f64);
}
