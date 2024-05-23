use std::rc::Rc;
use truck_geometry::Point2;

pub struct Circle {
    pub center: Rc<Point2>,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Rc<Point2>, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }
}
