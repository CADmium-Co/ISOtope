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

    pub fn start_angle(&self) -> f64 {
        match self {
            Segment::Line(line) => {
                let start = line.start().borrow().data();
                let end = line.end().borrow().data();
                (end.y - start.y).atan2(end.x - start.x)
            }
            Segment::Arc(arc) => arc.start_angle(),
        }
    }

    pub fn end_angle(&self) -> f64 {
        match self {
            Segment::Line(_line) => self.start_angle(),
            Segment::Arc(arc) => arc.end_angle(),
        }
    }

    pub fn reverse_equals(&self, other: &Self) -> bool {
        self == &other.reverse()
    }

    pub fn equals_or_reverse_equals(&self, other: &Self) -> bool {
        self == other || self.reverse_equals(other)
    }
}
