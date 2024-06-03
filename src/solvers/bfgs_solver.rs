use std::ops::DerefMut;
use std::{cell::RefCell, error::Error, rc::Rc};

use nalgebra::{DMatrix, UniformNorm};

use crate::sketch::Sketch;
use crate::solvers::line_search::line_search_wolfe;

use super::Solver;

pub struct BFGSSolver {
    max_iterations: usize,
    min_loss: f64,
    gradient_threshold: f64,
}

impl BFGSSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 1000,
            min_loss: 1e-16,
            gradient_threshold: 1e-8,
        }
    }

    pub fn new_with_params(max_iterations: usize, min_loss: f64, gradient_threshold: f64) -> Self {
        Self {
            max_iterations,
            min_loss,
            gradient_threshold,
        }
    }
}

impl Solver for BFGSSolver {
    fn solve(&self, sketch: Rc<RefCell<Sketch>>) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;

        let mut h = DMatrix::identity(
            sketch.borrow().get_data().len(),
            sketch.borrow().get_data().len(),
        );

        let mut data = sketch.borrow().get_data();
        while iterations < self.max_iterations {
            let loss = sketch.borrow_mut().get_loss();
            if loss < self.min_loss {
                break;
            }

            let gradient = sketch.borrow_mut().get_gradient();
            if !gradient.iter().all(|x| x.is_finite()) {
                return Err("gradient contains non-finite values".into());
            }

            if gradient.apply_norm(&UniformNorm) < self.gradient_threshold {
                break;
            }

            let p = -(&h) * &gradient;
            if !p.iter().all(|x| x.is_finite()) {
                return Err("search direction contains non-finite values".into());
            }

            let alpha = line_search_wolfe(sketch.borrow_mut().deref_mut(), &p, &gradient)?;

            let s = alpha * &p;

            let new_data = &data + &s;
            sketch.borrow_mut().set_data(new_data.clone());
            data = new_data;

            let new_gradient = sketch.borrow_mut().get_gradient();
            let y = &new_gradient - &gradient;

            let mut s_dot_y = s.dot(&y);
            if s_dot_y.abs() < 1e-16 {
                // println!("Warning: s_dot_y is too small");
                s_dot_y += 1e-6;
            }

            let hy = &h * &y;
            let factor = (s_dot_y + y.dot(&hy)) / (s_dot_y * s_dot_y);
            // h = 1.0*h + factor * s * s'
            h.ger(factor, &s, &s, 1.0);

            let hys_factor = -1.0 / s_dot_y;
            // h = 1.0*h - hy * s' / s_dot_y
            h.ger(hys_factor, &hy, &s, 1.0);
            // h = 1.0*h - s' * hy' / s_dot_y
            h.ger(hys_factor, &s, &hy, 1.0);

            iterations += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{
        examples::test_rectangle_rotated::RotatedRectangleDemo,
        solvers::{bfgs_solver::BFGSSolver, Solver},
    };

    #[test]
    pub fn test_bfgs_solver() -> Result<(), Box<dyn Error>> {
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

        rectangle.check(1e-5)
    }
}
