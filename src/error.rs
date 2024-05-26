use thiserror::Error;

#[derive(Error, Debug)]
pub enum ISOTopeError {
    // Sketch errors
    #[error("All references must be added to the sketch before the primitive")]
    MissingSketchReferences,
    #[error("The primitive is already in the sketch")]
    PrimitiveAlreadyInSketch,
    #[error("The constraint is already in the sketch")]
    ConstraintAlreadyInSketch,
}
