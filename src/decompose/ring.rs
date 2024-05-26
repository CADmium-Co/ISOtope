use crate::primitives::circle::Circle;

use super::segment::Segment;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}
