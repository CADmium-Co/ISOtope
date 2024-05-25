#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use nalgebra::Vector2;

    use crate::{
        constraints::{distance::{horizontal_distance_between_points::HorizontalDistanceBetweenPoints, vertical_distance_between_points::VerticalDistanceBetweenPoints}, fix_point::FixPoint, lines::{horizontal_line::HorizontalLine, vertical_line::VerticalLine}},
        primitives::{line::Line, point2::Point2},
        sketch::Sketch,
    };

    #[test]
    pub fn test_rectangle_axis_aligned() {
        let mut sketch = Sketch::new();

        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));

        sketch.add_primitive(point_a.clone());
        sketch.add_primitive(point_b.clone());
        sketch.add_primitive(point_c.clone());
        sketch.add_primitive(point_d.clone());

        let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
        let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
        let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

        sketch.add_primitive(line_a.clone());
        sketch.add_primitive(line_b.clone());
        sketch.add_primitive(line_c.clone());
        sketch.add_primitive(line_d.clone());

        // Fix point a to origin
        sketch.add_constraint(Rc::new(RefCell::new(FixPoint::new(
            point_a.clone(),
            Vector2::new(0.0, 0.0),
        ))));

        // Constrain line_a and line_c to be horizontal
        sketch.add_constraint(Rc::new(RefCell::new(
            HorizontalLine::new(line_a.clone()),
        )));
        sketch.add_constraint(Rc::new(RefCell::new(
            HorizontalLine::new(line_c.clone()),
        )));

        // Constrain line_b and line_d to be vertical
        sketch.add_constraint(Rc::new(RefCell::new(
            VerticalLine::new(line_b.clone()),
        )));
        sketch.add_constraint(Rc::new(RefCell::new(
            VerticalLine::new(line_d.clone()),
        )));

        // Constrain the length of line_a to 2
        sketch.add_constraint(Rc::new(RefCell::new(
           HorizontalDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
        )));

        // Constrain the length of line_b to 3
        sketch.add_constraint(Rc::new(RefCell::new(
           VerticalDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
        )));

        // Now solve the sketch
        sketch.solve(0.001, 100000);

        println!("point_a: {:?}", point_a.as_ref().borrow());
        println!("point_b: {:?}", point_b.as_ref().borrow());
        println!("point_c: {:?}", point_c.as_ref().borrow());
        println!("point_d: {:?}", point_d.as_ref().borrow());

        assert!(
            (point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 0.001
        );
        assert!(
            (point_b.as_ref().borrow().data() - Vector2::new(2.0, 0.0)).norm() < 0.001
        );
        assert!(
            (point_c.as_ref().borrow().data() - Vector2::new(2.0, 3.0)).norm() < 0.001
        );
        assert!(
            (point_d.as_ref().borrow().data() - Vector2::new(0.0, 3.0)).norm() < 0.001
        );
    }
}
