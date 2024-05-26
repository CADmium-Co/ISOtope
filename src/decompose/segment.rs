use super::arc::Arc;
use super::line::Line;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Segment {
    Line(Line),
    Arc(Arc),
}

impl Segment {
    pub fn reverse(&self) -> Self {
        match self {
            Segment::Line(line) => Segment::Line(line.reverse()),
            Segment::Arc(arc) => Segment::Arc(arc.reverse()),
        }
    }

    pub fn get_start(&self) -> u64 {
        match self {
            Segment::Line(line) => line.start,
            Segment::Arc(arc) => arc.start,
        }
    }

    pub fn get_end(&self) -> u64 {
        match self {
            Segment::Line(line) => line.end,
            Segment::Arc(arc) => arc.end,
        }
    }

    pub fn continues(&self, prior_segment: &Segment) -> bool {
        // determines if this segment continues the prior segment
        prior_segment.get_end() == self.get_start()
    }

    pub fn equals_or_reverse_equals(&self, other: &Self) -> bool {
        self == other || self == &other.reverse()
    }

    pub fn reverse_equals(&self, other: &Self) -> bool {
        self == &other.reverse()
    }
}
