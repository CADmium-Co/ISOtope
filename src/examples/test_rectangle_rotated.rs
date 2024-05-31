use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::{
    constraints::{
        angle_between_points::AngleBetweenPoints,
        distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints,
        fix_point::FixPoint, lines::perpendicular_lines::PerpendicularLines, ConstraintCell,
    },
    primitives::{line::Line, point2::Point2, PrimitiveCell},
    sketch::Sketch,
};

pub struct RotatedRectangleDemo {
    pub sketch: Rc<RefCell<Sketch>>,
    pub point_a: Rc<RefCell<Point2>>,
    pub point_b: Rc<RefCell<Point2>>,
    pub point_c: Rc<RefCell<Point2>>,
    pub point_d: Rc<RefCell<Point2>>,
    pub point_reference: Rc<RefCell<Point2>>,
}

impl RotatedRectangleDemo {
    pub fn new() -> Self {
        let sketch = Rc::new(RefCell::new(Sketch::new()));

        // This time we have to choose some random start points to break the symmetry
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.1)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.3, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(0.3, 0.3)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.1, 0.3)));

        let point_reference = Rc::new(RefCell::new(Point2::new(1.0, 0.0)));

        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_a.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_b.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_c.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_d.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Point2(point_reference.clone()))
            .unwrap();

        let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
        let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
        let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_a.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_b.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_c.clone()))
            .unwrap();
        sketch
            .borrow_mut()
            .add_primitive(PrimitiveCell::Line(line_d.clone()))
            .unwrap();

        // Fix point a to origin
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(point_a.clone(), Vector2::new(0.0, 0.0)),
            ))))
            .unwrap();

        // Constrain line_a and line_b to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_a.clone(), line_b.clone()),
            ))))
            .unwrap();

        // Constrain line_b and line_c to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_b.clone(), line_c.clone()),
            ))))
            .unwrap();

        // Constrain line_c and line_d to be perpendicular
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::PerpendicularLines(Rc::new(RefCell::new(
                PerpendicularLines::new(line_c.clone(), line_d.clone()),
            ))))
            .unwrap();

        // // Constrain line_d and line_a to be perpendicular
        // sketch.borrow_mut().add_constraint(Rc::new(RefCell::new(PerpendicularLines::new(
        //     line_d.clone(),
        //     line_a.clone(),
        // ))));

        // Constrain the length of line_a to 2
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                EuclidianDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
            ))))
            .unwrap();

        // Constrain the length of line_b to 3
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                EuclidianDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
            ))))
            .unwrap();

        // Fix reference point
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(point_reference.clone(), Vector2::new(1.0, 0.0)),
            ))))
            .unwrap();

        // Constrain rotation of line_a to 45 degrees
        sketch
            .borrow_mut()
            .add_constraint(ConstraintCell::AngleBetweenPoints(Rc::new(RefCell::new(
                AngleBetweenPoints::new(
                    point_reference.clone(),
                    point_b.clone(),
                    point_a.clone(),
                    f64::to_radians(45.0),
                ),
            ))))
            .unwrap();

        Self {
            sketch,
            point_a,
            point_b,
            point_c,
            point_d,
            point_reference,
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;
    use std::ops::DerefMut;

    use crate::{
        examples::test_rectangle_rotated::RotatedRectangleDemo,
        solvers::{bfgs_solver::BFGSSolver, Solver},
    };

    #[test]
    pub fn test_rectangle_rotated() {
        let rectangle = RotatedRectangleDemo::new();

        // Now solve the sketch
        let solver = BFGSSolver::new();
        solver
            .solve(rectangle.sketch.borrow_mut().deref_mut())
            .unwrap();

        println!("point_a: {:?}", rectangle.point_a.as_ref().borrow());
        println!("point_b: {:?}", rectangle.point_b.as_ref().borrow());
        println!("point_c: {:?}", rectangle.point_c.as_ref().borrow());
        println!("point_d: {:?}", rectangle.point_d.as_ref().borrow());
        println!(
            "point_reference: {:?}",
            rectangle.point_reference.as_ref().borrow()
        );

        assert!(
            (rectangle.point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 1e-5
        );
        assert!(
            (rectangle.point_b.as_ref().borrow().data()
                - Vector2::new(f64::sqrt(2.0), -f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_c.as_ref().borrow().data()
                - Vector2::new(5.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_d.as_ref().borrow().data()
                - Vector2::new(3.0 / f64::sqrt(2.0), 3.0 / f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_reference.as_ref().borrow().data() - Vector2::new(1.0, 0.0)).norm()
                < 1e-5
        );
    }
}
