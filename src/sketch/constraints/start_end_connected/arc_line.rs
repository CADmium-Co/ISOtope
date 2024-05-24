use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;

use crate::sketch::{constraints::Constraint, primitives::{arc::Arc, line::Line}};

// This is a sketch constraint that connects the end of the first line to the start of the following line.
#[derive(Debug)]
pub struct ArcLineStartEndConnected {
    pub first_arc: Rc<RefCell<Arc>>,
    pub following_line: Rc<RefCell<Line>>,
}

impl ArcLineStartEndConnected {
    pub fn new(first_arc: Rc<RefCell<Arc>>, following_line: Rc<RefCell<Line>>) -> Self {
        Self {
            first_arc,
            following_line,
        }
    }
}

impl Constraint for ArcLineStartEndConnected {
    fn loss_value(&self) -> f64 {
        let first_line_end = self.first_arc.borrow().end_point();
        let following_line_start = &self.following_line.borrow().start();

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        dx * dx + dy * dy
    }

    fn update_gradient(&mut self) {
        let first_line_end = self.first_arc.borrow().end_point();
        let following_line_start = self.following_line.borrow().start();

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        let grad_arc_end = self.first_arc.borrow().end_point_gradient();
        let grad_loss = SMatrix::<f64, 1, 2>::new(dx, dy);
        let grad = grad_loss * grad_arc_end;

        self.first_arc
            .as_ref()
            .borrow_mut()
            .add_to_gradient(grad);
        self.following_line
            .as_ref()
            .borrow_mut()
            .add_to_gradient(-dx, -dy, 0.0, 0.0);
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sketch::{
        constraints::start_end_connected::arc_line::ArcLineStartEndConnected, point2::Point2,
        primitives::{arc::Arc, line::Line}, Sketch,
    };

    #[test]
    fn test_arc_line() {
        let mut sketch = Sketch::new();

        let arc1 = Rc::new(RefCell::new(Arc::new(
            Point2::new(0.0, 0.0),
            1.0,
            false,
            0.0,
            std::f64::consts::PI,
        )));
        let line2 = Rc::new(RefCell::new(Line::new(
            Point2::new(3.0, 4.0),
            Point2::new(5.0, 6.0),
        )));
        sketch.add_primitive(arc1.clone());
        sketch.add_primitive(line2.clone());

        let constr1 = ArcLineStartEndConnected::new(arc1.clone(), line2.clone());
        sketch.add_constraint(Rc::new(RefCell::new(constr1)));

        sketch.solve(0.001, 10000);

        println!("arc1: {:?}", arc1.as_ref().borrow());
        println!("arc1 end point: {:?}", arc1.as_ref().borrow().end_point());
        println!("line2: {:?}", line2.as_ref().borrow());

        assert!((arc1.as_ref().borrow().end_point().x - line2.as_ref().borrow().start().x).abs() < 1e-6);
        assert!((arc1.as_ref().borrow().end_point().y - line2.as_ref().borrow().start().y).abs() < 1e-6);
    }
}
