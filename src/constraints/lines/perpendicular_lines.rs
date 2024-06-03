use std::{cell::RefCell, rc::Rc};

use nalgebra::Matrix2;
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::{
    constraints::ConstraintLike,
    primitives::{line::Line, PrimitiveCell},
};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct PerpendicularLines {
    line1: Rc<RefCell<Line>>,
    line2: Rc<RefCell<Line>>,
}

impl PerpendicularLines {
    pub fn new(line1: Rc<RefCell<Line>>, line2: Rc<RefCell<Line>>) -> Self {
        Self { line1, line2 }
    }

    pub fn line1(&self) -> Rc<RefCell<Line>> {
        self.line1.clone()
    }

    pub fn set_line1(&mut self, line1: Rc<RefCell<Line>>) {
        self.line1 = line1;
    }

    pub fn line2(&self) -> Rc<RefCell<Line>> {
        self.line2.clone()
    }

    pub fn set_line2(&mut self, line2: Rc<RefCell<Line>>) {
        self.line2 = line2;
    }
}

impl ConstraintLike for PerpendicularLines {
    fn references(&self) -> Vec<PrimitiveCell> {
        vec![
            PrimitiveCell::Line(self.line1.clone()),
            PrimitiveCell::Line(self.line2.clone()),
        ]
    }

    fn loss_value(&self) -> f64 {
        let start1 = self.line1.borrow().start().borrow().data();
        let end1 = self.line1.borrow().end().borrow().data();
        let start2 = self.line2.borrow().start().borrow().data();
        let end2 = self.line2.borrow().end().borrow().data();

        let dir1 = (end1 - start1).normalize();
        let dir2 = (end2 - start2).normalize();
        if !dir1.x.is_finite() || !dir1.y.is_finite() || !dir2.x.is_finite() || !dir2.y.is_finite()
        {
            return 0.0;
        }

        let dot_product = dir1.dot(&dir2);
        0.5 * dot_product * dot_product
    }

    fn update_gradient(&mut self) {
        let start1 = self.line1.borrow().start().borrow().data();
        let end1 = self.line1.borrow().end().borrow().data();
        let start2 = self.line2.borrow().start().borrow().data();
        let end2 = self.line2.borrow().end().borrow().data();

        let dir1 = end1 - start1;
        let dir1_norm = dir1.normalize(); // dir1 / dir1.norm();
        let dir2 = end2 - start2;
        let dir2_norm = dir2.normalize();
        if !dir1_norm.x.is_finite()
            || !dir1_norm.y.is_finite()
            || !dir2_norm.x.is_finite()
            || !dir2_norm.y.is_finite()
        {
            return;
        }

        let dot_product = dir1_norm.dot(&dir2_norm);
        let _loss = 0.5 * dot_product * dot_product;

        let grad_from_dot_product = dot_product;
        let grad_dot_product_from_dir1_norm = dir2_norm.transpose();
        let grad_dot_product_from_dir2_norm = dir1_norm.transpose();
        let grad_dir1_norm_from_dir1 =
            (Matrix2::identity() - dir1 * dir1.transpose() / dir1.norm_squared()) / dir1.norm();
        let grad_dir2_norm_from_dir2 =
            (Matrix2::identity() - dir2 * dir2.transpose() / dir2.norm_squared()) / dir2.norm();

        let grad_start1 = self.line1.borrow().start_gradient();
        let grad_end1 = self.line1.borrow().end_gradient();
        let grad_start2 = self.line2.borrow().start_gradient();
        let grad_end2 = self.line2.borrow().end_gradient();

        self.line1.borrow_mut().add_to_gradient(
            (grad_from_dot_product
                * grad_dot_product_from_dir1_norm
                * grad_dir1_norm_from_dir1
                * (grad_end1 - grad_start1))
                .as_view(),
        );

        self.line2.borrow_mut().add_to_gradient(
            (grad_from_dot_product
                * grad_dot_product_from_dir2_norm
                * grad_dir2_norm_from_dir2
                * (grad_end2 - grad_start2))
                .as_view(),
        );
    }

    fn get_type(&self) -> crate::constraints::Constraint {
        crate::constraints::Constraint::PerpendicularLines(self.clone())
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use nalgebra::Vector2;

    use crate::{
        constraints::{
            lines::perpendicular_lines::PerpendicularLines, ConstraintCell, ConstraintLike,
        },
        primitives::{line::Line, point2::Point2, PrimitiveCell},
        sketch::Sketch,
        solvers::{gradient_based_solver::GradientBasedSolver, Solver},
    };

    #[test]
    fn test_zero_vec_normalized_does_not_panic() {
        let mut v = Vector2::<f64>::zeros();
        v = v.normalize();
        assert!(v.x.is_nan());
        assert!(v.y.is_nan());
    }

    #[test]
    fn test_perpendicular_line() {
        let mut sketch = Sketch::new();
        let line1_start = Rc::new(RefCell::new(Point2::new(3.0, 4.0)));
        let line1_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line1 = Rc::new(RefCell::new(Line::new(
            line1_start.clone(),
            line1_end.clone(),
        )));
        sketch
            .add_primitive(PrimitiveCell::Point2(line1_start.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(line1_end.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line1.clone()))
            .unwrap();

        let line2_start = Rc::new(RefCell::new(Point2::new(0.0, 4.0)));
        let line2_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line2 = Rc::new(RefCell::new(Line::new(
            line2_start.clone(),
            line2_end.clone(),
        )));

        sketch
            .add_primitive(PrimitiveCell::Point2(line2_start.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(line2_end.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line2.clone()))
            .unwrap();

        let constr1 = Rc::new(RefCell::new(PerpendicularLines::new(
            line1.clone(),
            line2.clone(),
        )));
        sketch
            .add_constraint(ConstraintCell::PerpendicularLines(constr1.clone()))
            .unwrap();

        sketch.check_gradients(1e-6, constr1.clone(), 1e-6);
        let solver = GradientBasedSolver::new();
        solver.solve(&mut sketch).unwrap();

        println!(
            "line1_dir: {:?}",
            (line1_end.as_ref().borrow().data() - line1_start.as_ref().borrow().data()).normalize()
        );
        println!(
            "line2_dir: {:?}",
            (line2_end.as_ref().borrow().data() - line2_start.as_ref().borrow().data()).normalize()
        );

        println!("line1: {:?}", line1.as_ref().borrow());
        println!("line2: {:?}", line2.as_ref().borrow());

        println!("loss: {}", constr1.as_ref().borrow().loss_value());

        assert!(constr1.as_ref().borrow().loss_value() < 0.01);
    }
}
