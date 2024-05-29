use std::{cell::RefCell, error::Error, rc::Rc};

use crate::sketch::Sketch;

pub mod bfgs_solver;
pub mod gauss_newton_solver;
pub mod gradient_based_solver;
pub mod levenberg_marquardt;

pub trait Solver {
    fn solve(&self, sketch: Rc<RefCell<Sketch>>) -> Result<(), Box<dyn Error>>;
}
