use serde::{Deserialize, Serialize};

use crate::primitives::circle::Circle;

use super::segment::Segment;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}
