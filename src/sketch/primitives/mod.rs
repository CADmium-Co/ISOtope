pub mod line;
pub mod arc;
pub mod circle;

use self::line::Line;
use self::arc::Arc;
use self::circle::Circle;

pub enum SketchPrimitives{
    Line(Line),
    // Arc(Arc),
    // Circle(Circle),
}

impl SketchPrimitives {
    pub fn num_parameters(&self) -> usize {
        match self {
            SketchPrimitives::Line(l) => l.num_parameters(),
            // SketchPrimitives::Arc(a) => a.num_parameters(),
            // SketchPrimitives::Circle(c) => c.num_parameters(),
        }
    }
}

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a FIXED NUMBER of parameters.
pub trait Parametric<const N: usize> {
    fn from_data(data: [f64; N]) -> Self;
    fn data(&self) -> [f64; N];
    fn num_parameters(&self) -> usize {
        N
    }
}
