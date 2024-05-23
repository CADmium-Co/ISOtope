use std::rc::Rc;
use truck_geometry::Point2;

pub struct Arc {
    pub start: Rc<Point2>,
    pub end: Rc<Point2>,
    pub through_point: Point2,
}

impl Arc {
    pub fn new(start: Rc<Point2>, end: Rc<Point2>, through_point: Point2) -> Self {
        Self {
            start,
            end,
            through_point,
        }
    }
}
