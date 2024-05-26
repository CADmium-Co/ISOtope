use std::{cell::RefCell, rc::Rc};

use crate::sketch::Sketch;

pub struct GradientBasedSolver {
    pub sketch: Rc<RefCell<Sketch>>,
    pub max_iterations: usize,
    pub min_grad: f64,
    pub step_size: f64,
}

impl GradientBasedSolver {
    pub fn new(sketch: Rc<RefCell<Sketch>>) -> Self {
        Self {
            sketch,
            max_iterations: 10000,
            min_grad: 1e-6,
            step_size: 1e-3,
        }
    }

    pub fn new_with_params(
        sketch: Rc<RefCell<Sketch>>,
        max_iterations: usize,
        min_grad: f64,
        step_size: f64,
    ) -> Self {
        Self {
            sketch,
            max_iterations,
            min_grad,
            step_size,
        }
    }

    pub fn solve(&self) {
        let mut iterations = 0;
        let mut grad_norm = f64::INFINITY;

        while iterations < self.max_iterations && grad_norm > self.min_grad {
            self.sketch.borrow_mut().update();
            let mut data = self.sketch.borrow().get_data();
            let gradient = self.sketch.borrow().get_gradient();

            grad_norm = gradient.norm();
            data -= self.step_size * gradient;

            self.sketch.borrow_mut().set_data(data);

            iterations += 1;
        }
    }
}
