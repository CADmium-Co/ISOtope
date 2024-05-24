pub mod sketch;

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sketch::{
        constraints::start_end_connected::line_line::LineLineStartEndConnected, point2::Point2,
        primitives::line::Line, Sketch,
    };

    #[test]
    fn test_gradient_descent() {
        let mut sketch = Sketch::new();

        let line1 = Rc::new(RefCell::new(Line::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 1.0),
        )));
        let line2 = Rc::new(RefCell::new(Line::new(
            Point2::new(3.0, 4.0),
            Point2::new(5.0, 6.0),
        )));
        sketch.add_primitive(line1.clone());
        sketch.add_primitive(line2.clone());

        let constr1 = LineLineStartEndConnected::new(line1.clone(), line2.clone());
        sketch.add_constraint(Rc::new(RefCell::new(constr1)));

        sketch.solve(0.001, 10000);

        println!("line1: {:?}", line1.as_ref().borrow());
        println!("line2: {:?}", line2.as_ref().borrow());

        assert!((line1.as_ref().borrow().end().x - line2.as_ref().borrow().start().x).abs() < 1e-6);
        assert!((line1.as_ref().borrow().end().y - line2.as_ref().borrow().start().y).abs() < 1e-6);
    }
}
