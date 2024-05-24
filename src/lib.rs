pub mod sketch;

// Run some tests
#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, cell::RefCell, rc::Rc};

    use crate::sketch::{constraints::{start_end_connected::line_line::LineLineStartEndConnected, Constraint}, point2::Point2, primitives::{line::Line, Parametric}, Sketch};

    #[test]
    fn test_gradient_descent() {
        let mut sketch = Sketch::new();

        // let line1 = Box::new(Line::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0)));
        // let line2 = Box::new(Line::new(Point2::new(3.0, 4.0), Point2::new(5.0, 6.0)));
        // sketch.add_primitive(Rc::new(line1 as Box<dyn Parametric>));
        // sketch.add_primitive(Rc::new(line2 as Box<dyn Parametric>));

        // let constr1 = LineLineStartEndConnected::new(Rc::new(line1), Rc::new(line2));
        // sketch.add_constraint(Rc::new(Box::new(constr1) as Box<dyn Constraint>));

        // for _ in 0..100 {
        //     sketch.step(0.01);
        // }

        let line1 = Rc::new(RefCell::new(Line::new(Point2::new(0.0, 0.0), Point2::new(1.0, 1.0))));
        let line2 = Rc::new(RefCell::new(Line::new(Point2::new(3.0, 4.0), Point2::new(5.0, 6.0))));
        sketch.add_primitive(line1.clone());
        sketch.add_primitive(line2.clone());

        let constr1 = LineLineStartEndConnected::new(line1.clone(), line2.clone());
        sketch.add_constraint(Rc::new(RefCell::new(constr1)));

        sketch.step(0.0);
        println!("line1: {:?}", line1.as_ref().borrow());
        println!("line2: {:?}", line2.as_ref().borrow());

        for _ in 0..10000 {
            sketch.step(0.001);
        }

        println!("line1: {:?}", line1.as_ref().borrow());
        println!("line2: {:?}", line2.as_ref().borrow());

        assert!((line1.as_ref().borrow().end().x - line2.as_ref().borrow().start().x).abs() < 1e-6);
        assert!((line1.as_ref().borrow().end().y - line2.as_ref().borrow().start().y).abs() < 1e-6);

    }
}