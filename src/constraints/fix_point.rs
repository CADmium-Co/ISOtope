use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::{
    constraints::Constraint,
    primitives::point2::Point2,
};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug)]
pub struct FixPoint {
    point: Rc<RefCell<Point2>>,

    desired_pos: Vector2<f64>,
}

impl FixPoint {
    pub fn new(point: Rc<RefCell<Point2>>, desired_pos: Vector2<f64>) -> Self {
        Self { point, desired_pos }
    }

    pub fn point(&self) -> Rc<RefCell<Point2>> {
        self.point.clone()
    }

    pub fn set_point(&mut self, point: Rc<RefCell<Point2>>) {
        self.point = point;
    }

    pub fn desired_pos(&self) -> Vector2<f64> {
        self.desired_pos
    }

    pub fn set_desired_pos(&mut self, desired_pos: Vector2<f64>) {
        self.desired_pos = desired_pos;
    }
}

impl Constraint for FixPoint {
    fn references(&self) -> Vec<Rc<RefCell<dyn crate::primitives::Parametric>>> {
        vec![self.point.clone()]
    }

    fn loss_value(&self) -> f64 {
        let point = self.point.borrow().data();
        let d = point - self.desired_pos;
        0.5 * d.norm_squared()
    }

    fn update_gradient(&mut self) {
        let point = self.point.borrow().data();
        let d = point - self.desired_pos;

        let grad = d.transpose();
        self.point.borrow_mut().add_to_gradient(grad.as_view());
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use nalgebra::Vector2;

    use crate::{
        constraints::{fix_point::FixPoint, Constraint}, primitives::point2::Point2, sketch::Sketch,
    };

    #[test]
    fn test_fix_point() {
        let mut sketch = Sketch::new();

        let point = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));
        sketch.add_primitive(point.clone());

        let constr1 = Rc::new(RefCell::new(FixPoint::new(point.clone(), Vector2::new(1.0, 1.0))));
        sketch.add_constraint(constr1.clone());

        sketch.solve(0.001, 100000);

        println!("point: {:?}", point.as_ref().borrow());
        assert!(
            constr1.borrow().loss_value() < 0.001,
        );
    }
}
