pub mod primitives;

use core::panic;
use std::{collections::VecDeque, rc::Rc};

use truck_geometry::{Point2};

use self::primitives::{line::Line, SketchPrimitives};

pub struct Sketch {
    pub points: VecDeque<Rc<Point2>>,

    pub edges: VecDeque<SketchPrimitives>,
}

impl Sketch {
    pub fn new() -> Self {
        Self {
            points: VecDeque::new(),
            edges: VecDeque::new(),
        }
    }

    pub fn add_vertex(&mut self, point: Point2) {
        self.points.push_back(Rc::new(point));
    }

    pub fn add_primitive(&mut self, primitive: SketchPrimitives) {
        self.edges.push_back(primitive);
    }
}

