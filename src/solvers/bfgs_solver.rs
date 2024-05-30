use std::{cell::RefCell, error::Error, rc::Rc};

use nalgebra::{DMatrix, UniformNorm};

use crate::sketch::Sketch;

use super::Solver;

const WOLFE_C1: f64 = 1e-4;
const WOLFE_C2: f64 = 0.9;

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
        let mut alpha = 1.0;
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

            let m = gradient.dot(&p);
            if m > 0.0 {
                return Err("search direction is not a descent direction".into());
            }

            let curvature_condition = WOLFE_C2 * m;
            while alpha > 1e-16 {
                let new_data = &data + alpha * &p;
                sketch.borrow_mut().set_data(new_data);
                let new_loss = sketch.borrow_mut().get_loss();
                // Sufficient decrease condition
                if new_loss <= loss + WOLFE_C1 * alpha * m {
                    // Curvature condition
                    let new_gradient = sketch.borrow_mut().get_gradient();
                    let curvature = p.dot(&new_gradient);
                    if curvature >= curvature_condition {
                        break;
                    }
                    alpha *= 1.5;
                } else {
                    alpha *= 0.5;
                }
            }
            if alpha < 1e-16 {
                return Err("could not find a suitable step size".into());
            }

            let s = alpha * &p;

            let new_data = &data + &s;
            sketch.borrow_mut().set_data(new_data.clone());
            data = new_data;

            let new_gradient = sketch.borrow_mut().get_gradient();
            let y = &new_gradient - &gradient;

            let mut s_dot_y = s.dot(&y);
            if s_dot_y.abs() < 1e-16 {
                println!("Warning: s_dot_y is too small");
                s_dot_y += 1e-6;
            }
            let factor = s_dot_y + (y.transpose() * &h * &y)[(0, 0)];
            let new_h = &h + factor * (&s * s.transpose()) / (s_dot_y * s_dot_y)
                - (&h * &y * s.transpose() + &s * &y.transpose() * &h) / s_dot_y;
            h = new_h;

            iterations += 1;
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
