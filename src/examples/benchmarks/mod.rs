use std::{cell::RefCell, rc::Rc};

use crate::sketch::Sketch;

pub mod stairs_with_lines_benchmark;

pub trait BenchmarkFactory {
    fn new_benchmark(n: usize) -> Box<dyn Benchmark>;
}

pub trait Benchmark {
    fn get_sketch(&self) -> Rc<RefCell<Sketch>>;
    fn check(&self, eps: f64) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::solvers::gradient_based_solver::GradientBasedSolver;

    use super::stairs_with_lines_benchmark::StairsWithLinesBenchmarkFactory;

    #[ignore]
    #[test]
    // Run the benchmark manually with `cargo test --release test_benchmark -- --ignored`
    pub fn test_benchmark() {
        let benchmarks = vec![StairsWithLinesBenchmarkFactory];
        let solvers = vec![GradientBasedSolver::new()];
    }
}
