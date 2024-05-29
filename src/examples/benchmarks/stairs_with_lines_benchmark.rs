use std::{cell::RefCell, rc::Rc};

use crate::{
    constraints::{
        distance::horizontal_distance_between_points::HorizontalDistanceBetweenPoints,
        lines::horizontal_line::HorizontalLine, ConstraintCell,
    },
    primitives::{line::Line, point2::Point2, PrimitiveCell},
    sketch::Sketch,
};

use super::{Benchmark, BenchmarkFactory};

// This creates a stairs with lines problem that has a lot of constraints
// It tries to build a sketch like this with n steps (e.g. 6 steps):
//          _
//        _|
//      _|
//    _|
//  _|
// |

pub struct StairsWithLinesBenchmarkFactory;

impl BenchmarkFactory for StairsWithLinesBenchmarkFactory {
    fn new_benchmark(&self, n: usize) -> Box<dyn Benchmark> {
        let sketch = Rc::new(RefCell::new(Sketch::new()));

        let mut point_references = Vec::new();
        for _i in 0..n {
            let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
            sketch
                .borrow_mut()
                .add_primitive(PrimitiveCell::Point2(point.clone()))
                .unwrap();
            point_references.push(point);
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

            if i % 2 == 0 {
                sketch
                    .borrow_mut()
                    .add_constraint(ConstraintCell::HorizontalDistance(Rc::new(RefCell::new(
                        HorizontalDistanceBetweenPoints::new(
                            point_references[i].clone(),
                            point_references[i + 1].clone(),
                            0.8,
                        ),
                    ))))
                    .unwrap();

                sketch
                    .borrow_mut()
                    .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
                        HorizontalLine::new(line.clone()),
                    ))))
                    .unwrap();
            } else {
                sketch
                    .borrow_mut()
                    .add_constraint(ConstraintCell::HorizontalDistance(Rc::new(RefCell::new(
                        HorizontalDistanceBetweenPoints::new(
                            point_references[i].clone(),
                            point_references[i + 1].clone(),
                            0.8,
                        ),
                    ))))
                    .unwrap();

                sketch
                    .borrow_mut()
                    .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
                        HorizontalLine::new(line.clone()),
                    ))))
                    .unwrap();
            }
        }

        Box::new(StairsWithLinesBenchmark {
            sketch,
            point_references,
        })
    }
}

pub struct StairsWithLinesBenchmark {
    sketch: Rc<RefCell<Sketch>>,
    point_references: Vec<Rc<RefCell<Point2>>>,
}

impl Benchmark for StairsWithLinesBenchmark {
    fn check(&self, eps: f64) -> bool {
        for i in 0..self.point_references.len() - 1 {
            let point = self.point_references[i].as_ref().borrow();
            let true_x = (i / 2) as f64 * 0.8;
            let true_y = ((i + 1) / 2) as f64 * 0.8;
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
