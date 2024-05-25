use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;

use crate::{constraints::Constraint, primitives::line::Line};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug)]
pub struct ParallelLines {
    line1: Rc<RefCell<Line>>,
    line2: Rc<RefCell<Line>>,
}

impl ParallelLines {
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

impl Constraint for ParallelLines {
    fn references(&self) -> Vec<Rc<RefCell<dyn crate::primitives::Parametric>>> {
        vec![self.line1.clone(), self.line2.clone()]
    }

    fn loss_value(&self) -> f64 {
        let start1 = self.line1.borrow().start().borrow().data();
        let end1 = self.line1.borrow().end().borrow().data();
        let start2 = self.line2.borrow().start().borrow().data();
        let end2 = self.line2.borrow().end().borrow().data();

        let dir1 = end1 - start1;
        let dir2 = end2 - start2;

        let cross_product = dir1.x * dir2.y - dir1.y * dir2.x;
        0.5 * cross_product * cross_product
    }

    fn update_gradient(&mut self) {
        let start1 = self.line1.borrow().start().borrow().data();
        let end1 = self.line1.borrow().end().borrow().data();
        let start2 = self.line2.borrow().start().borrow().data();
        let end2 = self.line2.borrow().end().borrow().data();

        let dir1 = end1 - start1;
        let dir2 = end2 - start2;

        let cross_product = dir1.x * dir2.y - dir1.y * dir2.x;
        let _loss = 0.5 * cross_product * cross_product;

        let grad_from_cross_product = cross_product;
        let grad_cross_product_from_dir1 = SMatrix::<f64, 1, 2>::from_row_slice(&[dir2.y, -dir2.x]);
        let grad_cross_product_from_dir2 = SMatrix::<f64, 1, 2>::from_row_slice(&[-dir1.y, dir1.x]);

        let grad_start1 = self.line1.borrow().start_gradient();
        let grad_end1 = self.line1.borrow().end_gradient();
        let grad_start2 = self.line2.borrow().start_gradient();
        let grad_end2 = self.line2.borrow().end_gradient();

        self.line1.borrow_mut().add_to_gradient(
            (grad_from_cross_product * grad_cross_product_from_dir1 * (grad_end1 - grad_start1))
                .as_view(),
        );

        self.line2.borrow_mut().add_to_gradient(
            (grad_from_cross_product * grad_cross_product_from_dir2 * (grad_end2 - grad_start2))
                .as_view(),
        );
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::{lines::parallel_lines::ParallelLines, Constraint},
        primitives::{line::Line, point2::Point2},
        sketch::Sketch,
    };

    #[test]
    fn test_parallel_line() {
        let mut sketch = Sketch::new();

        let line1_start = Rc::new(RefCell::new(Point2::new(3.0, 4.0)));
        let line1_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line1 = Rc::new(RefCell::new(Line::new(
            line1_start.clone(),
            line1_end.clone(),
        )));
        sketch.add_primitive(line1_start.clone());
        sketch.add_primitive(line1_end.clone());
        sketch.add_primitive(line1.clone());

        let line2_start = Rc::new(RefCell::new(Point2::new(0.0, 4.0)));
        let line2_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line2 = Rc::new(RefCell::new(Line::new(
            line2_start.clone(),
            line2_end.clone(),
        )));

        sketch.add_primitive(line2_start.clone());
        sketch.add_primitive(line2_end.clone());
        sketch.add_primitive(line2.clone());

        let constr1 = Rc::new(RefCell::new(ParallelLines::new(
            line1.clone(),
            line2.clone(),
        )));
        sketch.add_constraint(constr1.clone());

        sketch.solve(0.001, 100000);

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

        assert!(constr1.as_ref().borrow().loss_value() < 0.001);
    }
}
