use crate::sketch::point2::Point2;

pub struct Arc {
    pub start: Point2,
    pub end: Point2,
    pub through_point: Point2,
}

impl Arc {
    pub fn new(start: Point2, end: Point2, through_point: Point2) -> Self {
        Self {
            start,
            end,
            through_point,
        }
    }
}
