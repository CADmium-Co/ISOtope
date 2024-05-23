pub mod primitives;
pub mod constraints;
pub mod point2;
pub mod gradient_buffer;

use std::collections::VecDeque;


use self::primitives::SketchPrimitives;

pub struct Sketch {
    pub edges: VecDeque<SketchPrimitives>,
}

impl Sketch {
    pub fn new() -> Self {
        Self {
            edges: VecDeque::new(),
        }
    }
    pub fn add_primitive(&mut self, primitive: SketchPrimitives) {
        self.edges.push_back(primitive);
    }
}

