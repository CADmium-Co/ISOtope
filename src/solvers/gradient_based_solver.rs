use std::error::Error;

use crate::sketch::Sketch;

use super::Solver;

pub struct GradientBasedSolver {
    max_iterations: usize,
    min_grad: f64,
    step_size: f64,
}

impl Default for GradientBasedSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientBasedSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 10000,
            min_grad: 1e-6,
            step_size: 1e-3,
        }
    }

    pub fn new_with_params(max_iterations: usize, min_grad: f64, step_size: f64) -> Self {
        Self {
            max_iterations,
            min_grad,
            step_size,
        }
    }
}

impl Solver for GradientBasedSolver {
    fn solve(&self, sketch: &mut Sketch) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;
        let mut grad_norm = f64::INFINITY;

        while iterations < self.max_iterations && grad_norm > self.min_grad {
            let mut data = sketch.get_data();
            let gradient = sketch.get_gradient();

            grad_norm = gradient.norm();
            data -= self.step_size * gradient;

            sketch.set_data(data);

            iterations += 1;
        }

        Ok(())
    }
}
