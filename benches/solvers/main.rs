use std::ops::DerefMut;
use std::time::Duration;
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

struct BenchmarkResult {
    solved: bool,
    duration: std::time::Duration,
    error: f64,
}

struct SolverResults {
    solver: String,
    results: Vec<BenchmarkResult>,
}

struct BenchmarkResults {
    n: usize,
    primitives: usize,
    constraints: usize,
    results: Vec<SolverResults>,
}

const N_REPEATS: usize = 1;

fn test_benchmark1() -> Vec<(String, Vec<BenchmarkResults>)> {
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

    let mut all_results = vec![];
    for (benchmark_name, benchmark) in benchmarks.iter() {
        let mut bench_results = vec![];
        for n in &[3, 5, 10, 30, 50, 100, 300] {
            let b = benchmark.new_benchmark(*n);
            let primitives = b.get_sketch().borrow().get_num_primitives();
            let constraints = b.get_sketch().borrow().get_num_constraints();

            let mut n_results = BenchmarkResults {
                n: *n,
                primitives,
                constraints,
                results: vec![],
            };

            for (solver_name, solver) in solvers.iter() {
                let mut solver_results = SolverResults {
                    solver: solver_name.to_string(),
                    results: vec![],
                };
                for _ in 0..N_REPEATS {
                    // Measure the time it takes to solve the benchmark
                    let benchmark = benchmark.new_benchmark(*n);
                    let sketch = benchmark.get_sketch();
                    let start = std::time::Instant::now();
                    solver.solve(sketch.borrow_mut().deref_mut()).unwrap();
                    let duration = start.elapsed();
                    let solved = benchmark.check(1e-6);
                    let error = sketch.borrow_mut().get_loss();
                    solver_results.results.push(BenchmarkResult {
                        solved,
                        duration,
                        error,
                    });
                }
                n_results.results.push(solver_results);
            }
            bench_results.push(n_results);
        }
        all_results.push((benchmark_name.to_string(), bench_results));
    }
    all_results
}

fn print_results(results: &[BenchmarkResults]) {
    let (max_n, max_primitives, max_constraints) = results.iter().fold((0, 0, 0), |a, b| {
        (a.0.max(b.n), a.1.max(b.primitives), a.2.max(b.constraints))
    });

    let n_len = max_n.to_string().len().max(1);
    let primitives_len = max_primitives.to_string().len().max(10);
    let constraints_len = max_constraints.to_string().len().max(11);

    let solver_len = results.iter().fold(7, |a, b| {
        a.max(b.results.iter().fold(0, |a, b| a.max(b.solver.len())))
    });

    let header = format!(
        "{:^n_len$} {:^primitives_len$} {:^constraints_len$} {:^solver_len$}  Error      Duration",
        "n",
        "Primitives",
        "Constraints",
        "Solver",
        n_len = n_len,
        primitives_len = primitives_len,
        constraints_len = constraints_len,
        solver_len = solver_len
    );
    let header_len = header.len();
    println!("{}", "━".repeat(header_len));
    println!("{}", header);

    for n_result in results.iter() {
        println!("{}", "─".repeat(header_len));
        let middle_row = (n_result.results.len() - 1) / 2;

        for (i, solver_result) in n_result.results.iter().enumerate() {
            let prefix = if i == middle_row {
                format!(
                    "{:^n_len$} {:^primitives_len$} {:^constraints_len$}",
                    n_result.n,
                    n_result.primitives,
                    n_result.constraints,
                    n_len = n_len,
                    primitives_len = primitives_len,
                    constraints_len = constraints_len,
                )
            } else {
                " ".repeat(n_len + primitives_len + constraints_len + 2)
            };

            let (all_solved, min_duration, max_error) = solver_result
                .results
                .iter()
                .fold((true, Duration::new(u64::MAX, 0), 0.0f64), |a, b| {
                    (a.0 && b.solved, a.1.min(b.duration), a.2.max(b.error))
                });
            println!(
                "{} {:<solver_len$} {:<8.2e} {} {:>6} ms",
                prefix,
                solver_result.solver,
                max_error,
                if all_solved { " " } else { "✗" },
                min_duration.as_millis(),
                solver_len = solver_len
            );
        }
    }
    println!("{}", "━".repeat(header_len));
    println!();
}

fn main() {
    let res = test_benchmark1();
    for (name, results) in res.iter() {
        println!("Benchmark: {}", name);
        print_results(results);
    }
}
