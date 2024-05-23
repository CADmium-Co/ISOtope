
use super::primitives::SketchPrimitives;

pub struct GradientBuffer {
    pub primitives: Vec<SketchPrimitives>,
    pub gradient: Vec<f64>,
}

impl GradientBuffer {
    pub fn new(primitives: Vec<SketchPrimitives>) -> Self {
        let n = primitives.iter().map(|p| p.num_parameters()).sum();

        Self {
            primitives,
            gradient: vec![0.0; n],
        }
    }

    pub fn zero_gradient(&mut self) {
        for g in self.gradient.iter_mut() {
            *g = 0.0;
        }
    }

    pub fn get_parameter_index<const N: usize>(primitive: &SketchPrimitives) -> [f64; N] {
        match primitive {
            SketchPrimitives::Line(l) => l.data(),
            // SketchPrimitives::Arc(a) => a.data(),
            // SketchPrimitives::Circle(c) => c.data(),
        }
    }
}