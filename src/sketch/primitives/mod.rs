pub mod line;
pub mod arc;
pub mod circle;

use self::line::Line;
// use self::arc::Arc;
// use self::circle::Circle;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a FIXED NUMBER of parameters.
pub trait Parametric {
    fn zero_gradient(&mut self);
    fn step(&mut self, step_size: f64);
}
