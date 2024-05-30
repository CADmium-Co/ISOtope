use crate::sketch::Sketch;
use nalgebra::DVector;
use std::error::Error;

const WOLFE_C1: f64 = 1e-4;
const WOLFE_C2: f64 = 0.9;

pub(crate) fn line_search_wolfe(
    sketch: &mut Sketch,
    direction: &DVector<f64>,
    gradient: &DVector<f64>,
) -> Result<f64, Box<dyn Error>> {
    let mut alpha = 1.0;
    let m = gradient.dot(direction);
    if m >= 0.0 {
        return Err("line search failed: search direction is not a descent direction".into());
    }
    let curvature_condition = WOLFE_C2 * m;
    let loss = sketch.get_loss();
    let x0 = sketch.get_data();
    while alpha > 1e-16 {
        let data = &x0 + alpha * direction;
        sketch.set_data(data);
        let new_loss = sketch.get_loss();
        // Sufficent decrease condition
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
    Err("line search failed: alpha is too small".into())
}
