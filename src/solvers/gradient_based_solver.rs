use std::ops::DerefMut;
use std::{cell::RefCell, error::Error, rc::Rc};

use crate::sketch::Sketch;
use crate::solvers::line_search::line_search_wolfe;

use super::Solver;

pub struct GradientBasedSolver {
    max_iterations: usize,
    min_loss: f64,
    min_grad: f64,
}

impl GradientBasedSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 10000,
            min_loss: 1e-14,
            min_grad: 1e-10,
        }
    }

    pub fn new_with_params(max_iterations: usize, min_loss: f64, min_grad: f64) -> Self {
        Self {
            max_iterations,
            min_loss,
            min_grad,
        }
    }
}

impl Solver for GradientBasedSolver {
    fn solve(&self, sketch: Rc<RefCell<Sketch>>) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;

        let mut gradient = sketch.borrow_mut().get_gradient();
        let mut grad_norm = gradient.norm();
        let mut loss = sketch.borrow_mut().get_loss();
        while iterations < self.max_iterations {
            if grad_norm < self.min_grad {
                break;
            }
            if loss < self.min_loss {
                break;
            }
            let mut data = sketch.borrow_mut().get_data();

            let direction = -&gradient;
            let alpha = line_search_wolfe(sketch.borrow_mut().deref_mut(), &direction, &gradient)?;
            // data = data + alpha * direction
            data.axpy(alpha, &direction, 1.0);
            sketch.borrow_mut().set_data(data);

            // Update metrics
            loss = sketch.borrow_mut().get_loss();
            gradient = sketch.borrow_mut().get_gradient();
            grad_norm = gradient.norm();

            iterations += 1;
        }

        Ok(())
    }
}
