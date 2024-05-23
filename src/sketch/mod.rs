pub mod primitives;
pub mod constraints;
pub mod point2;
pub mod gradient_buffer;

use std::collections::VecDeque;


use self::primitives::SketchPrimitives;

pub struct Sketch<'a> {
    pub edges: VecDeque<SketchPrimitives<'a>>,
}

impl<'a> Sketch<'a> {
    pub fn new() -> Self {
        Self {
            edges: VecDeque::new(),
        }
    }
    pub fn add_primitive(&mut self, primitive: SketchPrimitives<'a>) {
        self.edges.push_back(primitive);
    }
}

