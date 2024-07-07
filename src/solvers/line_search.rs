use crate::sketch::Sketch;
use nalgebra::DVector;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LineSearchError {
    #[error("line search failed: search direction is not a descent direction")]
    NotDescentDirection,
    #[error("line search failed: could not find a suitable step size")]
    SearchFailed,
}

const WOLFE_C1: f64 = 1e-4;
const WOLFE_C2: f64 = 0.9;
const MAX_ITER: usize = 15;

pub(crate) fn line_search_wolfe(
    sketch: &mut Sketch,
    direction: &DVector<f64>,
    gradient: &DVector<f64>,
) -> Result<f64, LineSearchError> {
    let mut alpha = 1.0;
    let m = gradient.dot(direction);
    if m >= 0.0 {
        return Err(LineSearchError::NotDescentDirection);
    }
    let curvature_condition = WOLFE_C2 * m;
    let loss = sketch.get_loss();
    let x0 = sketch.get_data();
    for _i in 0..MAX_ITER {
        let data = &x0 + alpha * direction;
        sketch.set_data(data);
        let new_loss = sketch.get_loss();
        // Sufficient decrease condition
        if new_loss <= loss + WOLFE_C1 * alpha * m {
            // Curvature condition
            let new_gradient = sketch.get_gradient();
            let curvature = new_gradient.dot(direction);
            if curvature >= curvature_condition {
                return Ok(alpha);
            }
            alpha *= 1.5;
        } else {
            alpha *= 0.5;
        }
    }
    Err(LineSearchError::SearchFailed)
}
