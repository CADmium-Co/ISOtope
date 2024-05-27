use approx::relative_eq;

use crate::primitives::{arc::Arc, circle::Circle, line::Line, point2::Point2, Primitive};

const INTERSECT_EPS: f64 = 1e-6;

pub fn intersections(a: Primitive, b: Primitive) -> Vec<nalgebra::Point2<f64>> {
    // We only deal with point intersections for now
    // In case two lines do overlap in parallel, we just treat the whole sketch as invalid
    // In case of two arcs or circles with same radius in same position, we treat the whole sketch as invalid, etc.
    match (a, b) {
        (Primitive::Line(l), Primitive::Point2(p)) | (Primitive::Point2(p), Primitive::Line(l)) => {
            line_intersect_point(&l, &p)
        }
        (Primitive::Line(l_a), Primitive::Line(l_b)) => line_intersect_line(&l_a, &l_b),
        (Primitive::Line(l), Primitive::Circle(c)) | (Primitive::Circle(c), Primitive::Line(l)) => {
            line_intersect_circle(&l, &c)
        }
        _ => todo!("Find all intersections between two primitives"),
    }
}

fn line_intersect_point(line: &Line, p: &Point2) -> Vec<nalgebra::Point2<f64>> {
    // Based on https://stackoverflow.com/a/328110
    let line_end = line.end_pt();
    let line_start = line.start_pt();
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
        return vec![]; // Early exit
    }

    // check that P lies between Start and End by comparing either it's x or y.
    let within_segment = if line_start.x != line_end.x {
        within_range(p.x(), line_start.x, line_end.x, INTERSECT_EPS)
    } else {
        within_range(p.y(), line_start.y, line_end.y, INTERSECT_EPS)
    };

    if !within_segment {
        vec![]
    } else {
        vec![point]
    }
}

fn line_intersect_line(line_a: &Line, line_b: &Line) -> Vec<nalgebra::Point2<f64>> {
    todo!()
}

fn line_intersect_circle(line: &Line, circle: &Circle) -> Vec<nalgebra::Point2<f64>> {
    todo!()
}
fn line_intersect_arc(line: &Line, arc: &Arc) -> Vec<nalgebra::Point2<f64>> {
    todo!()
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
