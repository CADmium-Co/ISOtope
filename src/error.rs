use thiserror::Error;

#[derive(Error, Debug)]
pub enum ISOTopeError {
    // Sketch errors
    #[error("All references must be added to the sketch before the primitive")]
    MissingSketchReferences,
    #[error("The primitive is already in the sketch")]
    PrimitiveAlreadyInSketch,
    #[error("The primitive with ID {0} is not in the sketch")]
    PrimitiveNotFound(u64),
    #[error("The constraint is already in the sketch")]
    ConstraintAlreadyInSketch,
    #[error("No such constraint in the sketch")]
    ConstraintNotFound,
}
