use crate::primitives::circle::Circle;

use super::segment::Segment;

#[derive(Debug)]
pub enum Ring {
    Circle(Circle),
    Segments(Vec<Segment>),
}
