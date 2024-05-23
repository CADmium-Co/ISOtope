pub mod line;
pub mod arc;
pub mod circle;

use self::line::Line;
use self::arc::Arc;
use self::circle::Circle;

pub enum SketchPrimitives<'a> {
    Line(Line<'a>),
    // Arc(Arc),
    // Circle(Circle),
}

impl<'a> SketchPrimitives<'a> {
    pub fn num_parameters(&self) -> usize {
        match self {
            SketchPrimitives::Line(l) => l.num_parameters(),
            // SketchPrimitives::Arc(a) => a.num_parameters(),
            // SketchPrimitives::Circle(c) => c.num_parameters(),
        }
    }
}

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a FIXED NUMBER of parameters.
pub trait Parametric<'a, const N: usize> {
    fn initialize(data: &'a mut [f64; N], gradient: &'a mut [f64; 4]) -> Self;
    fn num_parameters(&self) -> usize {
        N
    }
}
