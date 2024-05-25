use std::{cell::RefCell, rc::Rc};

use crate::{constraints::Constraint, primitives::point2::Point2};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug)]
pub struct EuclidianDistanceBetweenPoints {
    point1: Rc<RefCell<Point2>>,
    point2: Rc<RefCell<Point2>>,

    desired_distance: f64,
}

impl EuclidianDistanceBetweenPoints {
    pub fn new(
        point1: Rc<RefCell<Point2>>,
        point2: Rc<RefCell<Point2>>,
        desired_distance: f64,
    ) -> Self {
        Self {
            point1,
            point2,
            desired_distance,
        }
    }

    pub fn point1(&self) -> Rc<RefCell<Point2>> {
        self.point1.clone()
    }

    pub fn set_point1(&mut self, point1: Rc<RefCell<Point2>>) {
        self.point1 = point1;
    }

    pub fn point2(&self) -> Rc<RefCell<Point2>> {
        self.point2.clone()
    }

    pub fn set_point2(&mut self, point2: Rc<RefCell<Point2>>) {
        self.point2 = point2;
    }

    pub fn desired_distance(&self) -> f64 {
        self.desired_distance
    }

    pub fn set_desired_distance(&mut self, desired_distance: f64) {
        self.desired_distance = desired_distance;
    }

    pub fn current_distance(&self) -> f64 {
        let point1 = self.point1.borrow().data();
        let point2 = self.point2.borrow().data();

        let d = point1 - point2;
        d.norm()
    }
}

impl Constraint for EuclidianDistanceBetweenPoints {
    fn references(&self) -> Vec<Rc<RefCell<dyn crate::primitives::Parametric>>> {
        vec![self.point1.clone(), self.point2.clone()]
    }

    fn loss_value(&self) -> f64 {
        let distance = self.current_distance();
        let err = distance - self.desired_distance;
        0.5 * err * err
    }

    fn update_gradient(&mut self) {
        let point1 = self.point1.borrow().data();
        let point2 = self.point2.borrow().data();

        let d = point1 - point2;

        let distance = d.norm();
        let err = distance - self.desired_distance;
        let _loss = 0.5 * err * err;

        let grad_distance_from_d = d.transpose();
        let grad_err_from_distance = 1.0;
        let grad_loss_from_err = err;

        let grad_from_d = grad_loss_from_err * grad_err_from_distance * grad_distance_from_d;

        let grad_point1 = self.point1.borrow().point_gradient();
        let grad_point2 = self.point2.borrow().point_gradient();

        let grad_from_point1 = grad_from_d * grad_point1;
        let grad_from_point2 = -grad_from_d * grad_point2;

        self.point1
            .borrow_mut()
            .add_to_gradient(grad_from_point1.as_view());
        self.point2
            .borrow_mut()
            .add_to_gradient(grad_from_point2.as_view());
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::{
            distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints, Constraint,
        },
        primitives::point2::Point2,
        sketch::Sketch,
    };

    #[test]
    fn test_euclidian_distance_between_points() {
        let mut sketch = Sketch::new();

        let point_a = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.0, 1.0)));
        sketch.add_primitive(point_a.clone());
        sketch.add_primitive(point_b.clone());

        let constr1 = Rc::new(RefCell::new(EuclidianDistanceBetweenPoints::new(
            point_a.clone(),
            point_b.clone(),
            3.0,
        )));
        sketch.add_constraint(constr1.clone());

        sketch.check_gradients(1e-6, constr1.clone());
        sketch.solve(0.001, 100000);

        println!("point_a: {:?}", point_a.as_ref().borrow());
        println!("point_b: {:?}", point_b.as_ref().borrow());
        println!(
            "distance: {:?}",
            (point_a.as_ref().borrow().data() - point_b.as_ref().borrow().data()).norm()
        );

        assert!(constr1.borrow().loss_value() < 0.001,);
    }
}
