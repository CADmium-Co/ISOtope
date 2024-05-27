use approx::relative_eq;
use nalgebra::Vector2;

use crate::primitives::{arc::Arc, circle::Circle, line::Line, point2::Point2, Primitive};

const INTERSECT_EPS: f64 = 1e-6;

pub fn intersections(_a: Primitive, _b: Primitive) -> Vec<Vector2<f64>> {
    // We only deal with point intersections for now
    // In case two lines do overlap in parallel, we just treat the whole sketch as invalid
    // In case of two arcs or circles with same radius in same position, we treat the whole sketch as invalid, etc.
    todo!("Find all intersections between two primitives")
}

pub(crate) trait Intersect2D {
    fn intersect_point(&self, other: &Point2) -> Option<Vec<nalgebra::Point2<f64>>>;
    fn intersect_line(&self, other: &Line) -> Option<Vec<nalgebra::Point2<f64>>>;
    fn intersect_arc(&self, other: &Arc) -> Option<Vec<nalgebra::Point2<f64>>>;
    fn intersect_circle(&self, other: &Circle) -> Option<Vec<nalgebra::Point2<f64>>>;
}

impl Intersect2D for Line {
    /// Check that point `p` lies on the line segment defined by this [Line]
    fn intersect_point(&self, p: &Point2) -> Option<Vec<nalgebra::Point2<f64>>> {
        // Based on https://stackoverflow.com/a/328110
        let line_end = self.end_pt();
        let line_start = self.start_pt();
        let line_vector = line_end - line_start;
        let point: nalgebra::Point2<f64> = p.data().into();
        let point_vector = line_end - point;

        // p lies somewhere on the line if all three points are collinear.
        // check that cross product of End-Start and End-Other is 0
        let collinear = relative_eq!(
            line_vector.cross(&point_vector).magnitude(),
            0.0,
            epsilon = INTERSECT_EPS
        );
        if !collinear {
            return None; // Early exit
        }

        // check that P lies between Start and End by comparing either it's x or y.
        let within_segment = if line_start.x != line_end.x {
            within_range(p.x(), line_start.x, line_end.x, INTERSECT_EPS)
        } else {
            within_range(p.y(), line_start.y, line_end.y, INTERSECT_EPS)
        };

        if !within_segment {
            None
        } else {
            Some(vec![point])
        }
    }

    fn intersect_line(&self, other: &Line) -> Option<Vec<nalgebra::Point2<f64>>> {
        // easy
        None
    }

    fn intersect_arc(&self, other: &Arc) -> Option<Vec<nalgebra::Point2<f64>>> {
        // Up to two intersections
        None
    }

    fn intersect_circle(&self, other: &Circle) -> Option<Vec<nalgebra::Point2<f64>>> {
        // Up to two intersections
        None
    }
}

#[inline]
fn within_range(x: f64, a: f64, b: f64, epsilon: f64) -> bool {
    if a == b && b == x {
        return true;
    }
    if a < b {
        x >= a - epsilon && x <= b + epsilon
    } else {
        x >= b - epsilon && x <= a + epsilon
    }
}
