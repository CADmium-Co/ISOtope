pub mod primitives;
pub mod constraints;
pub mod point2;

use std::{cell::RefCell, collections::VecDeque, rc::Rc};


use self::{constraints::Constraint, primitives::Parametric};

pub struct Sketch {
    primitives: VecDeque<Rc<RefCell<dyn Parametric>>>,
    constraints: VecDeque<Rc<RefCell<dyn Constraint>>>,
}

impl Sketch {
    pub fn new() -> Self {
        Self {
            primitives: VecDeque::new(),
            constraints: VecDeque::new(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Rc<RefCell<dyn Parametric>>)
    {
        self.primitives.push_back(primitive);
    }

    pub fn add_constraint(&mut self, constraint: Rc<RefCell<dyn Constraint>>) {
        self.constraints.push_back(constraint);
    }

    pub fn step(&mut self, step_size: f64) {
        for primitive in self.primitives.iter_mut() {
            primitive.borrow_mut().zero_gradient();
        }

        for constraint in self.constraints.iter_mut() {
            constraint.borrow_mut().update_gradient();
        }

        for primitive in self.primitives.iter_mut() {
            primitive.borrow_mut().step(step_size);
        }
    }
}
