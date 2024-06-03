use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::{
    constraints::ConstraintLike,
    primitives::{arc::Arc, point2::Point2, PrimitiveCell},
};

// This is a sketch constraint that makes the start point of an arc coincident with a point.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
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

impl ConstraintLike for ArcStartPointCoincident {
    fn references(&self) -> Vec<PrimitiveCell> {
        vec![
            PrimitiveCell::Arc(self.arc.clone()),
            PrimitiveCell::Point2(self.point.clone()),
        ]
    }

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
        let grad_point = self.point.borrow().point_gradient();

        self.arc
            .borrow_mut()
            .add_to_gradient((gradient_constraint * grad_arc).as_view());
        self.point
            .borrow_mut()
            .add_to_gradient((-gradient_constraint * grad_point).as_view());
    }

    fn get_type(&self) -> crate::constraints::Constraint {
        crate::constraints::Constraint::ArcStartPointCoincident(self.clone())
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::{
            coincident::arc_start_point_coincident::ArcStartPointCoincident, ConstraintCell,
        },
        primitives::{arc::Arc, line::Line, point2::Point2, PrimitiveCell},
        sketch::Sketch,
        solvers::{gradient_based_solver::GradientBasedSolver, Solver},
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
        sketch
            .add_primitive(PrimitiveCell::Point2(center.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Arc(arc1.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(line2_start.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(line2_end.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line2.clone()))
            .unwrap();

        let constr1 = Rc::new(RefCell::new(ArcStartPointCoincident::new(
            arc1.clone(),
            line2_end.clone(),
        )));
        sketch
            .add_constraint(ConstraintCell::ArcStartPointCoincident(constr1.clone()))
            .unwrap();

        sketch.check_gradients(1e-6, constr1.clone(), 1e-5);
        let solver = GradientBasedSolver::new();
        solver.solve(&mut sketch).unwrap();

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
