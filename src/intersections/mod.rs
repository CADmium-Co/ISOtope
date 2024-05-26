use nalgebra::Vector2;

use crate::primitives::Primitive;

pub fn intersections(_a: Primitive, _b: Primitive) -> Vec<Vector2<f64>> {
    // We only deal with point intersections for now
    // In case two lines do overlap in parallel, we just treat the whole sketch as invalid
    // In case of two arcs or circles with same radius in same position, we treat the whole sketch as invalid, etc.
    todo!("Find all intersections between two primitives")
}
