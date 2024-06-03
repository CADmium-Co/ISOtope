use std::error::Error;

use crate::sketch::Sketch;

mod line_search;

pub mod bfgs_solver;
pub mod gauss_newton_solver;
pub mod gradient_based_solver;
pub mod levenberg_marquardt;

pub trait Solver {
    fn solve(&self, sketch: &mut Sketch) -> Result<(), Box<dyn Error>>;
}
