use std::{cell::RefCell, error::Error, rc::Rc};

use nalgebra::DMatrix;

use crate::sketch::Sketch;

use super::Solver;

pub struct BFGSSolver {
    max_iterations: usize,
    min_loss: f64,
    step_alpha: f64,
    alpha_search_steps: usize,
    // step_alpha_decay: f64,
}

impl BFGSSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 1000,
            min_loss: 1e-16,
            step_alpha: 1e-2,
            alpha_search_steps: 400,
            // step_alpha_decay: f64::powf(0.1, 1.0 / 1000.0),
        }
    }

    pub fn new_with_params(
        max_iterations: usize,
        min_loss: f64,
        step_alpha: f64,
        alpha_search_steps: usize,
        // step_alpha_decay: f64,
    ) -> Self {
        Self {
            max_iterations,
            min_loss,
            step_alpha,
            alpha_search_steps,
            // step_alpha_decay,
        }
    }
}

impl Solver for BFGSSolver {
    fn solve(&self, sketch: Rc<RefCell<Sketch>>) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;
        let mut loss = f64::INFINITY;

        let mut h = DMatrix::identity(
            sketch.borrow().get_data().len(),
            sketch.borrow().get_data().len(),
        );

        let mut data = sketch.borrow().get_data();
        let alpha = self.step_alpha;
        while iterations < self.max_iterations && loss > self.min_loss {
            // println!("Data: {:?}", data);
            let gradient = sketch.borrow_mut().get_gradient();

            loss = sketch.borrow_mut().get_loss();
            // println!("Loss: {:?}", loss);

            let p = -(&h) * &gradient;
            let mut best_alpha = f64::INFINITY;
            for i in 0..self.alpha_search_steps {
                let new_data = &data + alpha * i as f64 * &p;
                sketch.borrow_mut().set_data(new_data);
                let new_loss = sketch.borrow_mut().get_loss();
                if new_loss < loss {
                    best_alpha = alpha * i as f64;
                    loss = new_loss;
                }
            }

            if best_alpha >= alpha * self.alpha_search_steps as f64 * 0.8 {
                println!("Warning: step_alpha is too small or alpha_search_steps is too low");
            }

            let s = best_alpha * &p;

            let new_data = &data + &s;
            sketch.borrow_mut().set_data(new_data.clone());
            data = new_data;

            let new_gradient = sketch.borrow_mut().get_gradient();
            let y = &new_gradient - &gradient;

            let s_dot_y = s.dot(&y);
            let factor = s_dot_y + (y.transpose() * &h * &y)[(0, 0)];
            let new_h = &h + factor * (&s * s.transpose()) / (s_dot_y * s_dot_y)
                - (&h * &y * s.transpose() + &s * &y.transpose() * &h) / s_dot_y;
            h = new_h;

            iterations += 1;
            // alpha *= self.step_alpha_decay;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;

    use crate::{
        examples::test_rectangle_rotated::RotatedRectangleDemo,
        solvers::{bfgs_solver::BFGSSolver, Solver},
    };

    #[test]
    pub fn test_bfgs_solver() {
        let rectangle = RotatedRectangleDemo::new();

        // Now solve the sketch
        let solver = BFGSSolver::new();
        solver.solve(rectangle.sketch.clone()).unwrap();

        println!("loss: {:?}", rectangle.sketch.borrow_mut().get_loss());
        println!("point_a: {:?}", rectangle.point_a.as_ref().borrow());
        println!("point_b: {:?}", rectangle.point_b.as_ref().borrow());
        println!("point_c: {:?}", rectangle.point_c.as_ref().borrow());
        println!("point_d: {:?}", rectangle.point_d.as_ref().borrow());
        println!(
            "point_reference: {:?}",
            rectangle.point_reference.as_ref().borrow()
        );

        assert!(
            (rectangle.point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 1e-5
        );
        assert!(
            (rectangle.point_b.as_ref().borrow().data()
                - Vector2::new(f64::sqrt(2.0), -f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_c.as_ref().borrow().data()
                - Vector2::new(5.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_d.as_ref().borrow().data()
                - Vector2::new(3.0 / f64::sqrt(2.0), 3.0 / f64::sqrt(2.0)))
            .norm()
                < 1e-5
        );
        assert!(
            (rectangle.point_reference.as_ref().borrow().data() - Vector2::new(1.0, 0.0)).norm()
                < 1e-5
        );
    }
}
