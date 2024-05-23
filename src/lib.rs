pub mod sketch;

// Run some tests
#[cfg(test)]
mod tests {
    use crate::sketch::{constraints::{start_end_connected::line_line::LineLineStartEndConnected, Constraints}, point2::Point2, primitives::line::Line, Sketch};

    #[test]
    fn test_gradient_descent() {
        let mut sketch = Sketch::<64>::new();

        let mut line1 = sketch.add_primitive::<Line>();
        line1.set_start(Point2::new(1.0, 2.0));
        line1.set_end(Point2::new(3.0, 4.0));
    
        let mut line2 = sketch.add_primitive::<Line>();
        line2.set_start(Point2::new(3.0, 4.0));
        line2.set_end(Point2::new(5.0, 6.0));

        let constr1 = LineLineStartEndConnected::new(&mut line1,&mut line2);
        sketch.add_constraint(Constraints::StartEndConnected(constr1));

        for _ in 0..100 {
            sketch.step(0.01);
        }

    }
}