use serde::{Deserialize, Serialize};

use crate::primitives::circle::Circle;

use super::segment::Segment;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}

impl Ring {
    pub fn signed_area(&self) -> f64 {
        match self {
            Ring::Circle(circle) => circle.radius().powi(2) * std::f64::consts::PI,
            Ring::Segments(segments) => {
                let mut area = 0.0;
                for segment in segments {
                    let start = segment.get_start();
                    let end = segment.get_end();
                    area += start.x * end.y - end.x * start.y;
                }
                area / 2.0
            }
        }
    }
}
