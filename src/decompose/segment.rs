use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

use crate::primitives::{arc::Arc, line::Line};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Segment {
    Line(Line),
    Arc(Arc),
}

impl Segment {
    pub fn get_start(&self) -> Vector2<f64> {
        match self {
            Segment::Line(line) => line.start().borrow().data(),
            Segment::Arc(arc) => arc.start_point(),
        }
    }

    pub fn get_end(&self) -> Vector2<f64> {
        match self {
            Segment::Line(line) => line.end().borrow().data(),
            Segment::Arc(arc) => arc.end_point(),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Segment::Line(line) => {
                Segment::Line(Line::new(line.end().clone(), line.start().clone()))
            }
            Segment::Arc(arc) => Segment::Arc(arc.reverse()),
        }
    }

    pub fn continues(&self, prior_segment: &Segment) -> bool {
        // determines if this segment continues the prior segment
        prior_segment.get_end() == self.get_start()
    }

    pub fn connects(&self, prior_segment: &Segment) -> bool {
        // determines if this segment connects to the prior segment
        prior_segment.get_end() == self.get_start() || prior_segment.get_end() == self.get_end()
    }
}
