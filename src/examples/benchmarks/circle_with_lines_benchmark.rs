use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

use crate::{
    constraints::{
        angle_between_points::AngleBetweenPoints,
        distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints,
        fix_point::FixPoint, ConstraintCell,
    },
    primitives::{line::Line, point2::Point2, PrimitiveCell},
    sketch::Sketch,
};

use super::{Benchmark, BenchmarkFactory};

// This creates a circle problem from lines
pub fn circle(n: usize) -> Vec<Vector2<f64>> {
    let mut points = Vec::new();
    for i in 0..n {
        let x = ((i + 1) / 2) as f64 * 0.8;
        let y = (i / 2) as f64 * 0.8;
        points.push(Vector2::new(x, y));
    }
    points
}

pub struct CirclesWithLinesBenchmarkFactory;

impl BenchmarkFactory for CirclesWithLinesBenchmarkFactory {
    fn new_benchmark(&self, n: usize) -> Box<dyn Benchmark> {
        let reference_points = circle(n);

        let sketch = Rc::new(RefCell::new(Sketch::new()));

        let mut point_references = Vec::new();
        for i in 0..n {
            let point = Rc::new(RefCell::new(Point2::new(0.0, (i as f64) / (n as f64))));
            sketch
                .borrow_mut()
                .add_primitive(PrimitiveCell::Point2(point.clone()))
                .unwrap();
            point_references.push(point);
        }

        for i in 0..n {
            sketch
                .borrow_mut()
                .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                    FixPoint::new(
                        point_references[i].clone(),
                        Vector2::new(reference_points[i].x, reference_points[i].y),
                    ),
                ))))
                .unwrap();
        }

        for i in 0..n - 1 {
            let line = Rc::new(RefCell::new(Line::new(
                point_references[i].clone(),
                point_references[i + 1].clone(),
            )));
            sketch
                .borrow_mut()
                .add_primitive(PrimitiveCell::Line(line.clone()))
                .unwrap();

            let distance = (reference_points[i + 1] - reference_points[i]).norm();
            sketch
                .borrow_mut()
                .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                    EuclidianDistanceBetweenPoints::new(
                        point_references[i].clone(),
                        point_references[i + 1].clone(),
                        distance,
                    ),
                ))))
                .unwrap();

            let angle = (&reference_points[i + 1] - &reference_points[i])
                .angle(&(&reference_points[(i + n - 1) % n] - &reference_points[i]));
            sketch
                .borrow_mut()
                .add_constraint(ConstraintCell::AngleBetweenPoints(Rc::new(RefCell::new(
                    AngleBetweenPoints::new(
                        point_references[i + 1].clone(),
                        point_references[(i + n - 1) % n].clone(),
                        point_references[i].clone(),
                        angle,
                    ),
                ))))
                .unwrap();
        }

        Box::new(CirclesWithLinesBenchmark {
            sketch,
            point_references,
        })
    }
}

pub struct CirclesWithLinesBenchmark {
    sketch: Rc<RefCell<Sketch>>,
    point_references: Vec<Rc<RefCell<Point2>>>,
}

impl Benchmark for CirclesWithLinesBenchmark {
    fn check(&self, eps: f64) -> bool {
        let reference_points = circle(self.point_references.len());
        for i in 0..self.point_references.len() {
            let point = self.point_references[i].borrow();
            let true_x = reference_points[i].x;
            let true_y = reference_points[i].y;
            if (point.x() - true_x).abs() > eps || (point.y() - true_y).abs() > eps {
                return false;
            }
        }
        true
    }

    fn get_sketch(&self) -> Rc<RefCell<Sketch>> {
        self.sketch.clone()
    }
}
