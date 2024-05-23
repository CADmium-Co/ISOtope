use truck_geometry::Point2;

pub struct Line {
    pub start: Point2,
    pub end: Point2,
}

impl Line {
    pub fn new(start: Point2, end: Point2) -> Self {
        Self {
            start,
            end,
        }
    }
}

