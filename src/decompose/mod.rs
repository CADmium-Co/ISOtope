use crate::sketch::Sketch;

use self::face::Face;

pub mod face;
pub mod ring;
pub mod segment;

pub fn decompose_sketch(sketch: &Sketch) -> Vec<Face> {
    for primitive in sketch.primitives() {
        let primitive = primitive.borrow().to_primitive();
    }

    todo!("Decompose the sketch into faces")
}
