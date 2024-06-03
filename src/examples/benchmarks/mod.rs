use std::{cell::RefCell, rc::Rc};

use crate::sketch::Sketch;

pub mod circle_with_lines_benchmark;
pub mod stairs_with_lines_benchmark;

pub trait BenchmarkFactory {
    fn new_benchmark(&self, n: usize) -> Box<dyn Benchmark>;
}

pub trait Benchmark {
    fn get_sketch(&self) -> Rc<RefCell<Sketch>>;
    fn check(&self, eps: f64) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::solvers::{
        bfgs_solver::BFGSSolver, gauss_newton_solver::GaussNewtonSolver,
        gradient_based_solver::GradientBasedSolver, levenberg_marquardt::LevenbergMarquardtSolver,
        Solver,
    };
    use std::ops::DerefMut;

    use super::{
        circle_with_lines_benchmark::CirclesWithLinesBenchmarkFactory,
        stairs_with_lines_benchmark::StairsWithLinesBenchmarkFactory, BenchmarkFactory,
    };

    #[ignore]
    #[test]
    // Run the benchmark manually with `cargo test --release test_benchmark1 -- --ignored --nocapture`
    pub fn test_benchmark1() {
        let benchmarks: Vec<(&str, Box<dyn BenchmarkFactory>)> = vec![
            (
                "CirclesWithLines",
                Box::new(CirclesWithLinesBenchmarkFactory),
            ),
            ("StairsWithLines", Box::new(StairsWithLinesBenchmarkFactory)),
        ];
        let solvers: Vec<(&str, Box<dyn Solver>)> = vec![
            (
                "GradientBasedSolver     ",
                Box::new(GradientBasedSolver::new()),
            ),
            (
                "GaussNewtonSolver       ",
                Box::new(GaussNewtonSolver::new()),
            ),
            (
                "LevenbergMarquardtSolver",
                Box::new(LevenbergMarquardtSolver::new()),
            ),
            ("BFGSSolver              ", Box::new(BFGSSolver::new())),
        ];

        for (benchmark_name, benchmark) in benchmarks.iter() {
            println!("Benchmark: {}", benchmark_name);
            for n in &[3, 5, 10] {
                for (solver_name, solver) in &solvers {
                    // Measure the time it takes to solve the benchmark
                    let benchmark = benchmark.new_benchmark(*n);
                    let sketch = benchmark.get_sketch();
                    let start = std::time::Instant::now();
                    solver.solve(sketch.borrow_mut().deref_mut()).unwrap();
                    let duration = start.elapsed();
                    let solved = benchmark.check(1e-6);
                    let error = sketch.borrow_mut().get_loss();
                    println!(
                        "n: {:4}, \tprimitives: {:4}, \tconstraints:{:4}, \tsolver: {},\tsolved: {},\terror: {:.2},\tduration: {}ms",
                        n,
                        sketch.borrow().get_num_primitives(),
                        sketch.borrow().get_num_constraints(),
                        solver_name,
                        solved,
                        error,
                        duration.as_millis()
                    );
                }
            }
        }
    }

    #[ignore]
    #[test]
    // Run the benchmark manually with `cargo test --release test_benchmark -- --ignored --nocapture`
    pub fn test_benchmark2() {
        let benchmarks: Vec<(&str, Box<dyn BenchmarkFactory>)> = vec![
            (
                "CirclesWithLines",
                Box::new(CirclesWithLinesBenchmarkFactory),
            ),
            ("StairsWithLines", Box::new(StairsWithLinesBenchmarkFactory)),
        ];
        let solvers: Vec<(&str, Box<dyn Solver>)> = vec![
            ("GradientBasedSolver", Box::new(GradientBasedSolver::new())),
            ("BFGSSolver         ", Box::new(BFGSSolver::new())),
        ];

        for (benchmark_name, benchmark) in benchmarks.iter() {
            println!("Benchmark: {}", benchmark_name);
            for n in &[30, 50, 100, 300] {
                for (solver_name, solver) in &solvers {
                    // Measure the time it takes to solve the benchmark
                    let benchmark = benchmark.new_benchmark(*n);
                    let sketch = benchmark.get_sketch();
                    let start = std::time::Instant::now();
                    solver.solve(sketch.borrow_mut().deref_mut()).unwrap();
                    let duration = start.elapsed();
                    let solved = benchmark.check(1e-6);
                    let error = sketch.borrow_mut().get_loss();
                    println!(
                        "n: {:4}, \tprimitives: {:4}, \tconstraints:{:4}, \tsolver: {},\tsolved: {},\terror: {:.2},\tduration: {}ms",
                        n,
                        sketch.borrow().get_num_primitives(),
                        sketch.borrow().get_num_constraints(),
                        solver_name,
                        solved,
                        error,
                        duration.as_millis()
                    );
                }
            }
        }
    }
}
