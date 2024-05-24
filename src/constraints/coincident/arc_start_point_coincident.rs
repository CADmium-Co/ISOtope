use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;

use crate::{
    constraints::Constraint,
    primitives::{arc::Arc, point2::Point2},
};

// This is a sketch constraint that makes the start point of an arc coincident with a point.
#[derive(Debug)]
pub struct ArcStartPointCoincident {
    arc: Rc<RefCell<Arc>>,
    point: Rc<RefCell<Point2>>,
}

impl ArcStartPointCoincident {
    pub fn new(arc: Rc<RefCell<Arc>>, point: Rc<RefCell<Point2>>) -> Self {
        Self { arc, point }
    }

    pub fn arc(&self) -> Rc<RefCell<Arc>> {
        self.arc.clone()
    }

    pub fn set_arc(&mut self, arc: Rc<RefCell<Arc>>) {
        self.arc = arc;
    }

    pub fn point(&self) -> Rc<RefCell<Point2>> {
        self.point.clone()
    }

    pub fn set_point(&mut self, point: Rc<RefCell<Point2>>) {
        self.point = point;
    }
}

impl Constraint for ArcStartPointCoincident {
    fn loss_value(&self) -> f64 {
        let arc_start = self.arc.borrow().start_point();
        let point = self.point.borrow().data();
        let dx = arc_start.x - point.x;
        let dy = arc_start.y - point.y;
        0.5 * (dx * dx + dy * dy)
    }

    fn update_gradient(&mut self) {
        let arc_start = self.arc.borrow().start_point();
        let point = self.point.borrow().data();
        let dx = arc_start.x - point.x;
        let dy = arc_start.y - point.y;

        let gradient_constraint = SMatrix::<f64, 1, 2>::from_row_slice(&[dx, dy]);

        let grad_arc = self.arc.borrow().start_point_gradient();
        let grad_point = self.point.borrow().gradient();

        self.arc
            .borrow_mut()
            .add_to_gradient((gradient_constraint * grad_arc).as_view());
        self.point
            .borrow_mut()
            .add_to_gradient((gradient_constraint * grad_point).as_view());
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::coincident::arc_start_point_coincident::ArcStartPointCoincident,
        primitives::{arc::Arc, line::Line, point2::Point2},
        sketch::Sketch,
    };

    #[test]
    fn test_arc_start_point_coincident() {
        let mut sketch = Sketch::new();

        let center = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let arc1 = Rc::new(RefCell::new(Arc::new(
            center.clone(),
            1.0,
            false,
            0.0,
            std::f64::consts::PI,
        )));
        let line2_start = Rc::new(RefCell::new(Point2::new(3.0, 4.0)));
        let line2_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line2 = Rc::new(RefCell::new(Line::new(
            line2_start.clone(),
            line2_end.clone(),
        )));
        sketch.add_primitive(center.clone());
        sketch.add_primitive(arc1.clone());
        sketch.add_primitive(line2_start.clone());
        sketch.add_primitive(line2_end.clone());
        sketch.add_primitive(line2.clone());

        let constr1 = ArcStartPointCoincident::new(arc1.clone(), line2_end.clone());
        sketch.add_constraint(Rc::new(RefCell::new(constr1)));

        sketch.solve(0.001, 100000);

        println!("arc1: {:?}", arc1.as_ref().borrow());
        println!(
            "arc1 start point: {:?}",
            arc1.as_ref().borrow().start_point()
        );
        println!("line2: {:?}", line2.as_ref().borrow());

        assert!(
            (arc1.as_ref().borrow().start_point().x - line2.as_ref().borrow().end().borrow().x())
                .abs()
                < 1e-6
        );
        assert!(
            (arc1.as_ref().borrow().start_point().y - line2.as_ref().borrow().end().borrow().y())
                .abs()
                < 1e-6
        );
    }
}
