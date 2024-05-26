#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use nalgebra::Vector2;

    use crate::{
        constraints::{
            angle_between_points::AngleBetweenPoints,
            distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints,
            fix_point::FixPoint, lines::perpendicular_lines::PerpendicularLines,
        },
        primitives::{line::Line, point2::Point2},
        sketch::Sketch,
    };

    #[test]
    pub fn test_rectangle_rotated() {
        let mut sketch = Sketch::new();

        // This time we have to choose some random start points to break the symmetry
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.1)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.3, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(0.3, 0.3)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.1, 0.3)));

        let point_reference = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));

        sketch.add_primitive(point_a.clone());
        sketch.add_primitive(point_b.clone());
        sketch.add_primitive(point_c.clone());
        sketch.add_primitive(point_d.clone());
        sketch.add_primitive(point_reference.clone());

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

        // Constrain line_a and line_b to be perpendicular
        sketch.add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
            line_a.clone(),
            line_b.clone(),
        ))));

        // Constrain line_b and line_c to be perpendicular
        sketch.add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
            line_b.clone(),
            line_c.clone(),
        ))));

        // Constrain line_c and line_d to be perpendicular
        sketch.add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
            line_c.clone(),
            line_d.clone(),
        ))));

        // // Constrain line_d and line_a to be perpendicular
        // sketch.add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
        //     line_d.clone(),
        //     line_a.clone(),
        // ))));

        // Constrain the length of line_a to 2
        sketch.add_constraint(Rc::new(RefCell::new(EuclidianDistanceBetweenPoints::new(
            point_a.clone(),
            point_b.clone(),
            2.0,
        ))));

        // Constrain the length of line_b to 3
        sketch.add_constraint(Rc::new(RefCell::new(EuclidianDistanceBetweenPoints::new(
            point_a.clone(),
            point_d.clone(),
            3.0,
        ))));

        // Fix reference point
        sketch.add_constraint(Rc::new(RefCell::new(FixPoint::new(
            point_reference.clone(),
            Vector2::new(1.0, 0.0),
        ))));

        // // Constrain rotation of line_a to 45 degrees
        sketch.add_constraint(Rc::new(RefCell::new(AngleBetweenPoints::new(
            point_reference.clone(),
            point_b.clone(),
            point_a.clone(),
            f64::to_radians(45.0),
        ))));

        // Now solve the sketch
        sketch.solve(0.01, 50000);

        println!("point_a: {:?}", point_a.as_ref().borrow());
        println!("point_b: {:?}", point_b.as_ref().borrow());
        println!("point_c: {:?}", point_c.as_ref().borrow());
        println!("point_d: {:?}", point_d.as_ref().borrow());
        println!("point_reference: {:?}", point_reference.as_ref().borrow());

        assert!((point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 0.01);
        assert!(
            (point_b.as_ref().borrow().data() - Vector2::new(f64::sqrt(2.0), -f64::sqrt(2.0)))
                .norm()
                < 0.01
        );
        assert!(
            (point_c.as_ref().borrow().data()
                - Vector2::new(5.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)))
            .norm()
                < 0.01
        );
        assert!(
            (point_d.as_ref().borrow().data()
                - Vector2::new(3.0 / f64::sqrt(2.0), 3.0 / f64::sqrt(2.0)))
            .norm()
                < 0.01
        );
    }
}
