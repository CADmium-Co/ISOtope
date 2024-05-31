#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use nalgebra::Vector2;

    use crate::{
        constraints::{
            distance::{
                horizontal_distance_between_points::HorizontalDistanceBetweenPoints,
                vertical_distance_between_points::VerticalDistanceBetweenPoints,
            },
            fix_point::FixPoint,
            lines::{horizontal_line::HorizontalLine, vertical_line::VerticalLine},
            ConstraintCell,
        },
        primitives::{line::Line, point2::Point2, PrimitiveCell},
        sketch::Sketch,
        solvers::{bfgs_solver::BFGSSolver, Solver},
    };

    #[test]
    pub fn test_rectangle_axis_aligned() {
        // Create a new empty sketch
        let mut sketch = Sketch::new();
        // Create four points
        let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_b = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_c = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        let point_d = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));

        // Add the points to the sketch
        sketch
            .add_primitive(PrimitiveCell::Point2(point_a.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_b.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_c.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Point2(point_d.clone()))
            .unwrap();

        // Create four lines based on the points
        let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
        let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
        let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
        let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

        // Add the lines to the sketch
        sketch
            .add_primitive(PrimitiveCell::Line(line_a.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line_b.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line_c.clone()))
            .unwrap();
        sketch
            .add_primitive(PrimitiveCell::Line(line_d.clone()))
            .unwrap();

        // Fix point a to origin
        sketch
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(point_a.clone(), Vector2::new(0.0, 0.0)),
            ))))
            .unwrap();

        // Constrain line_a and line_c to be horizontal
        sketch
            .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
                HorizontalLine::new(line_a.clone()),
            ))))
            .unwrap();
        sketch
            .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
                HorizontalLine::new(line_c.clone()),
            ))))
            .unwrap();

        // Constrain line_b and line_d to be vertical
        sketch
            .add_constraint(ConstraintCell::VerticalLine(Rc::new(RefCell::new(
                VerticalLine::new(line_b.clone()),
            ))))
            .unwrap();
        sketch
            .add_constraint(ConstraintCell::VerticalLine(Rc::new(RefCell::new(
                VerticalLine::new(line_d.clone()),
            ))))
            .unwrap();

        // Constrain the length of line_a to 2
        sketch
            .add_constraint(ConstraintCell::HorizontalDistance(Rc::new(RefCell::new(
                HorizontalDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
            ))))
            .unwrap();

        // Constrain the length of line_b to 3
        sketch
            .add_constraint(ConstraintCell::VerticalDistance(Rc::new(RefCell::new(
                VerticalDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
            ))))
            .unwrap();

        // Now solve the sketch
        let solver = BFGSSolver::new();
        solver.solve(&mut sketch).unwrap();

        println!("loss = {:?}", sketch.get_loss());
        println!("point_a: {:?}", point_a.as_ref().borrow());
        println!("point_b: {:?}", point_b.as_ref().borrow());
        println!("point_c: {:?}", point_c.as_ref().borrow());
        println!("point_d: {:?}", point_d.as_ref().borrow());

        assert!((point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 1e-8);
        assert!((point_b.as_ref().borrow().data() - Vector2::new(2.0, 0.0)).norm() < 1e-8);
        assert!((point_c.as_ref().borrow().data() - Vector2::new(2.0, 3.0)).norm() < 1e-8);
        assert!((point_d.as_ref().borrow().data() - Vector2::new(0.0, 3.0)).norm() < 1e-8);
    }
}
