use std::ops::DerefMut;
use std::{cell::RefCell, rc::Rc};

use isotope::sketch::Sketch;
use isotope::solvers::bfgs_solver::BFGSSolver;
use isotope::solvers::gradient_based_solver::GradientBasedSolver;
use isotope::solvers::Solver;

use crate::circle_with_lines_benchmark::CirclesWithLinesBenchmarkFactory;
use crate::stairs_with_lines_benchmark::StairsWithLinesBenchmarkFactory;

pub mod circle_with_lines_benchmark;
pub mod stairs_with_lines_benchmark;

pub trait BenchmarkFactory {
    fn new_benchmark(&self, n: usize) -> Box<dyn Benchmark>;
}

pub trait Benchmark {
    fn get_sketch(&self) -> Rc<RefCell<Sketch>>;
    fn check(&self, eps: f64) -> bool;
}

pub fn test_benchmark1() {
    let benchmarks: Vec<(&str, Box<dyn BenchmarkFactory>)> = vec![
        (
            "CirclesWithLines",
            Box::new(CirclesWithLinesBenchmarkFactory),
        ),
        ("StairsWithLines", Box::new(StairsWithLinesBenchmarkFactory)),
    ];
    let solvers: Vec<(&str, Box<dyn Solver>)> = vec![
        ("GradientBasedSolver", Box::new(GradientBasedSolver::new())),
        ("BFGSSolver", Box::new(BFGSSolver::new())),
    ];

    let max_length = solvers.iter().fold(6, |acc, (name, _)| acc.max(name.len()));

    for (benchmark_name, benchmark) in benchmarks.iter() {
        println!("Benchmark: {}", benchmark_name);
        for n in &[3, 5, 10, 30, 50, 100, 300] {
            let b = benchmark.new_benchmark(*n);
            let primatives = b.get_sketch().borrow().get_num_primitives();
            let constraints = b.get_sketch().borrow().get_num_constraints();
            println!(
                " ║\n ╟─ n: {:4}, primitives: {:4}, constraints:{:4}",
                n, primatives, constraints
            );
            println!(" ║  {}━━━━━━━━━━━━━━━━━━━━", "━".repeat(max_length));
            println!(" ║  {:^1$}   Error    Duration", "Solver", max_length);
            println!(" ║  {}────────────────────", "─".repeat(max_length));
            for (solver_name, solver) in solvers.iter() {
                // Measure the time it takes to solve the benchmark
                let benchmark = benchmark.new_benchmark(*n);
                let sketch = benchmark.get_sketch();
                let start = std::time::Instant::now();
                solver.solve(sketch.borrow_mut().deref_mut()).unwrap();
                let duration = start.elapsed();
                let solved = benchmark.check(1e-6);
                let error = sketch.borrow_mut().get_loss();
                println!(
                    " ║  {:<4$} {:<8.2e}{} {:>6} ms",
                    solver_name,
                    error,
                    if solved { " " } else { "✗" },
                    duration.as_millis(),
                    max_length,
                );
            }
            println!(" ║  {}━━━━━━━━━━━━━━━━━━━━", "━".repeat(max_length));
        }
        println!();
    }
}
fn main() {
    test_benchmark1();
}
