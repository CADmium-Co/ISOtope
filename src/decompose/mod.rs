use crate::{primitives::Primitive, sketch::Sketch};

use self::face::Face;

pub mod face;
pub mod ring;
pub mod segment;

pub fn decompose_sketch(sketch: &Sketch) -> Vec<Face> {
    let _primitives = sketch
        .primitives()
        .iter()
        .map(|p| p.borrow().to_primitive())
        .collect::<Vec<Primitive>>();
    // A primitive is now ether a Circle, Line, or Arc. Points can be ignored.
    // Now chain all consecutive primitives that are connected into a ring.
    // - Two primitives are connected if the end of the first primitive is the start of the second primitive.
    // For now, assume there is only one ring in the sketch, such that the construction of the faces is simple.
    // For the future, we will need a more complex algorithm that can handle multiple rings. But for the MVP, this is sufficient.

    todo!("Decompose the sketch into faces")
}
