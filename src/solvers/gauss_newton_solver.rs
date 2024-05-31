use std::error::Error;

use crate::sketch::Sketch;

use super::Solver;

pub struct GaussNewtonSolver {
    max_iterations: usize,
    min_loss: f64,
    step_size: f64,
    pseudo_inverse_eps: f64,
}

impl GaussNewtonSolver {
    pub fn new() -> Self {
        Self {
            max_iterations: 10000,
            min_loss: 1e-6,
            step_size: 1e-3,
            pseudo_inverse_eps: 1e-6,
        }
    }

    pub fn new_with_params(max_iterations: usize, min_loss: f64, step_size: f64) -> Self {
        Self {
            max_iterations,
            min_loss,
            step_size,
            pseudo_inverse_eps: 1e-6,
        }
    }
}

impl Solver for GaussNewtonSolver {
    fn solve(&self, sketch: &mut Sketch) -> Result<(), Box<dyn Error>> {
        let mut iterations = 0;
        let mut loss_sum = f64::INFINITY;

        while iterations < self.max_iterations && loss_sum > self.min_loss {
            let mut data = sketch.get_data();
            let losses = sketch.get_loss_per_constraint();
            loss_sum = losses.sum();
            let jacobian = sketch.get_jacobian();

            data -= (jacobian.transpose() * jacobian.clone())
                .clone()
                .pseudo_inverse(self.pseudo_inverse_eps)?
                * &jacobian.transpose()
                * &losses
                * self.step_size;

            sketch.set_data(data);

            iterations += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector2;
    use std::ops::DerefMut;

    use crate::{
        examples::test_rectangle_rotated::RotatedRectangleDemo,
        solvers::{gauss_newton_solver::GaussNewtonSolver, Solver},
    };

    #[test]
    pub fn test_gauss_newton_solver() {
        let rectangle = RotatedRectangleDemo::new();

        // Now solve the sketch
        let solver = GaussNewtonSolver::new_with_params(500, 1e-8, 1e0);
        solver
            .solve(rectangle.sketch.borrow_mut().deref_mut())
            .unwrap();

        println!("loss: {:?}", rectangle.sketch.borrow_mut().get_loss());
        println!("point_a: {:?}", rectangle.point_a.as_ref());
        println!("point_b: {:?}", rectangle.point_b.as_ref());
        println!("point_c: {:?}", rectangle.point_c.as_ref());
        println!("point_d: {:?}", rectangle.point_d.as_ref());
        println!("point_reference: {:?}", rectangle.point_reference.as_ref());

        assert!(
            (rectangle.point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 0.01
        );
        assert!(
            (rectangle.point_b.as_ref().borrow().data()
                - Vector2::new(f64::sqrt(2.0), -f64::sqrt(2.0)))
            .norm()
                < 0.1
        );
        assert!(
            (rectangle.point_c.as_ref().borrow().data()
                - Vector2::new(5.0 / f64::sqrt(2.0), 1.0 / f64::sqrt(2.0)))
            .norm()
                < 0.1
        );
        assert!(
            (rectangle.point_d.as_ref().borrow().data()
                - Vector2::new(3.0 / f64::sqrt(2.0), 3.0 / f64::sqrt(2.0)))
            .norm()
                < 0.1
        );
        assert!(
            (rectangle.point_reference.as_ref().borrow().data() - Vector2::new(1.0, 0.0)).norm()
                < 0.1
        );
    }
}
