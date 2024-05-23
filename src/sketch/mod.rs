pub mod primitives;
pub mod constraints;
pub mod point2;

use std::collections::VecDeque;


use self::{constraints::Constraints, primitives::{Parametric, SketchPrimitives}};

pub struct Sketch<'a, const N: usize> {
    data: [f64; N],
    gradient: [f64; N],

    edges: VecDeque<SketchPrimitives<'a>>,
    n: usize,

    constraints: VecDeque<Constraints<'a>>,
}

impl<'a, const N: usize> Sketch<'a, N> {
    pub fn new() -> Self {
        Self {
            data: [0.0; N],
            gradient: [0.0; N],
            edges: VecDeque::new(),
            n: 0,
            constraints: VecDeque::new(),
        }
    }

    pub fn add_primitive<P: Parametric<'a>>(&'a mut self) -> &mut P {
        let data = &mut self.data[self.n..self.n + P::num_parameters()];
        let gradient = &mut self.gradient.as_mut_slice()[self.n..self.n + P::num_parameters()];
        let primitive = P::initialize(data, gradient);
        self.edges.push_back(primitive.as_sketch_primitive());
        self.n += P::num_parameters();
        P::ref_from_sketch_primitive(self.edges.back_mut().unwrap())
    }

    pub fn add_constraint(&mut self, constraint: Constraints<'a>) {
        self.constraints.push_back(constraint);
    }

    pub fn step(&'a mut self, step_size: f64) {
        for i in 0..N {
            self.gradient[i] = 0.0;
        }

        for constraint in self.constraints.iter_mut() {
            constraint.constraint_mut().update_gradient();
        }

        for i in 0..N {
            self.data[i] -= step_size * self.gradient[i];
        }
    }
}

