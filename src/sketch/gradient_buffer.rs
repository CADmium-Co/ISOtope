
use super::primitives::SketchPrimitives;

pub struct GradientBuffer<'a> {
    pub primitives: Vec<SketchPrimitives<'a>>,
    pub gradient: Vec<f64>,
}

impl<'a> GradientBuffer<'a> {
    pub fn new(primitives: Vec<SketchPrimitives<'a>>) -> Self {
        let n = primitives.iter().map(|p| p.num_parameters()).sum();

        Self {
            primitives,
            gradient: vec![0.0; n],
        }
    }
}