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
    pub fn add_primitive<P: Parametric<'a, N2>, const N2: usize>(&'a mut self) {
        let slice1 = &mut self.data[self.n..self.n + N2];
        let data = slice1.try_into().unwrap();
        let slice2 = &mut self.gradient.as_mut_slice()[self.n..self.n + N2];
        let gradient = slice2.try_into().unwrap();
        let primitive = P::initialize(data, gradient);
        self.edges.push_back(primitive.as_sketch_primitive());
        self.n += N2;
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

