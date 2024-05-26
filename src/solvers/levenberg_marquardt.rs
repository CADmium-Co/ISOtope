use std::{cell::RefCell, rc::Rc};

use nalgebra::DMatrix;

use crate::sketch::Sketch;

pub struct LevenbergMarquardtSolver {
    sketch: Rc<RefCell<Sketch>>,
    max_iterations: usize,
    min_loss: f64,
    step_size: f64,
    pseudo_inverse_eps: f64,
    beta: f64,
}

impl LevenbergMarquardtSolver {
    pub fn new(sketch: Rc<RefCell<Sketch>>) -> Self {
        Self {
            sketch,
            max_iterations: 1000,
            min_loss: 1e-10,
            step_size: 1e-1,
            pseudo_inverse_eps: 1e-6,
            beta: 1e-5,
        }
    }

    pub fn new_with_params(
        sketch: Rc<RefCell<Sketch>>,
        max_iterations: usize,
        min_loss: f64,
        step_size: f64,
        beta: f64,
    ) -> Self {
        Self {
            sketch,
            max_iterations,
            min_loss,
            step_size,
            pseudo_inverse_eps: 1e-6,
            beta,
        }
    }

    pub fn solve(&self) -> Result<(), &'static str> {
        let mut iterations = 0;
        let mut loss_sum = f64::INFINITY;

        while iterations < self.max_iterations && loss_sum > self.min_loss {
            let mut data = self.sketch.borrow().get_data();
            let losses = self.sketch.borrow().get_loss_per_constraint();
            loss_sum = losses.sum();
            let jacobian = self.sketch.borrow_mut().get_jacobian();

            data -= (jacobian.transpose() * jacobian.clone()
                + self.beta * DMatrix::identity(jacobian.ncols(), jacobian.ncols()))
            .clone()
            .pseudo_inverse(self.pseudo_inverse_eps)?
                * &jacobian.transpose()
                * &losses
                * self.step_size;

            self.sketch.borrow_mut().set_data(data);

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
        solvers::levenberg_marquardt::LevenbergMarquardtSolver,
    };

    #[test]
    pub fn test_levenberg_marquardt_solver() {
        let rectangle = RotatedRectangleDemo::new();

        // Now solve the sketch
        let solver = LevenbergMarquardtSolver::new_with_params(
            rectangle.sketch.clone(),
            1000,
            1e-10,
            1e-1,
            1e-5,
        );
        solver.solve().unwrap();

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
