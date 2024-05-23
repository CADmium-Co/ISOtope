use std::rc::Rc;
use truck_geometry::Point2;

pub struct Line {
    pub start: Rc<Point2>,
    pub end: Rc<Point2>,
}

impl Line {
    pub fn new(start: Rc<Point2>, end: Rc<Point2>) -> Self {
        Self {
            start,
            end,
        }
    }
}

