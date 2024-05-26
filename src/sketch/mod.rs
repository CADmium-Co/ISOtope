use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use nalgebra::DVector;

use super::{constraints::Constraint, primitives::Parametric};

#[derive(Default)]
pub struct Sketch {
    primitives: VecDeque<Rc<RefCell<dyn Parametric>>>,
    constraints: VecDeque<Rc<RefCell<dyn Constraint>>>,
}

impl Sketch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_primitive(&mut self, primitive: Rc<RefCell<dyn Parametric>>) {
        // Make sure all referenced primitives are added to the sketch before the primitive
        for reference in primitive.borrow().references() {
            if !self.primitives.iter().any(|p| Rc::ptr_eq(p, &reference)) {
                panic!("All references must be added to the sketch before the primitive");
            }
        }
        // Check that the primitive is not already in the sketch
        if self.primitives.iter().any(|p| Rc::ptr_eq(p, &primitive)) {
            panic!("The primitive is already in the sketch");
        }
        // Add the primitive to the sketch
        self.primitives.push_back(primitive);
    }

    pub fn add_constraint(&mut self, constraint: Rc<RefCell<dyn Constraint>>) {
        // Make sure all referenced primitives are added to the sketch before the constraint
        for reference in constraint.borrow().references() {
            if !self.primitives.iter().any(|p| Rc::ptr_eq(p, &reference)) {
                panic!("All references must be added to the sketch before the constraint");
            }
        }
        // Make sure the constraint is not already in the sketch
        if self.constraints.iter().any(|c| Rc::ptr_eq(c, &constraint)) {
            panic!("The constraint is already in the sketch");
        }

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

    pub fn solve(&mut self, step_size: f64, max_steps: usize) {
        for i in 0..max_steps {
            self.step(step_size);

            let gradient_sum = self
                .primitives
                .iter()
                .map(|p| p.borrow().get_gradient().norm())
                .sum::<f64>();

            if gradient_sum < 1e-6 {
                println!("Converged after {} steps", i + 1);
                break;
            }
        }
    }

    // This function is used in test cases to check the gradients of the primitives
    pub fn check_gradients(
        &mut self,
        epsilon: f64,
        constraint: Rc<RefCell<dyn Constraint>>,
        check_epsilon: f64,
    ) {
        // Update all gradients
        self.step(0.0);

        // Compare to numerical gradients
        let constraint_loss = constraint.borrow().loss_value();
        for primitive in self.primitives.iter_mut() {
            let original_value = primitive.borrow().get_data();
            let analytical_gradient = primitive.borrow().get_gradient();
            let mut numerical_gradient = DVector::zeros(original_value.len());
            let n = primitive.borrow().get_data().len();
            assert!(n == analytical_gradient.len());
            for i in 0..n {
                let mut new_value = original_value.clone();
                new_value[i] += epsilon;
                primitive.borrow_mut().set_data(new_value.clone().as_view());
                let new_loss = constraint.borrow().loss_value();
                primitive
                    .borrow_mut()
                    .set_data(original_value.clone().as_view());
                numerical_gradient[i] = (new_loss - constraint_loss) / epsilon;
            }

            println!("Analytical gradient: {:?}", analytical_gradient);
            println!("Numerical gradient: {:?}", numerical_gradient);

            let error = (numerical_gradient - analytical_gradient).norm();
            println!("Error: {}", error);
            assert!(error < check_epsilon);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constraints::coincident::arc_end_point_coincident::ArcEndPointCoincident,
        primitives::{arc::Arc, point2::Point2},
    };

    use super::*;

    #[test]
    fn test_references_have_to_be_added_beforehand() {
        assert!(std::panic::catch_unwind(|| {
            let mut sketch = Sketch::new();

            let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
            let arc = Rc::new(RefCell::new(Arc::new(point, 1.0, true, 0.0, 1.0)));

            sketch.add_primitive(arc.clone());
        })
        .is_err());
    }

    #[test]
    fn test_primitive_cannot_be_added_twice() {
        assert!(std::panic::catch_unwind(|| {
            let mut sketch = Sketch::new();

            let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
            sketch.add_primitive(point.clone());
            sketch.add_primitive(point.clone());
        })
        .is_err());
    }

    #[test]
    fn test_constraint_references_have_to_be_added_beforehand() {
        assert!(std::panic::catch_unwind(|| {
            let mut sketch = Sketch::new();

            let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
            let arc = Rc::new(RefCell::new(Arc::new(point.clone(), 1.0, true, 0.0, 1.0)));

            sketch.add_primitive(point.clone());

            let constraint = Rc::new(RefCell::new(ArcEndPointCoincident::new(arc, point)));
            sketch.add_constraint(constraint);
        })
        .is_err());
    }

    #[test]
    fn test_constraint_cannot_be_added_twice() {
        assert!(std::panic::catch_unwind(|| {
            let mut sketch = Sketch::new();

            let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
            let arc = Rc::new(RefCell::new(Arc::new(point.clone(), 1.0, true, 0.0, 1.0)));

            sketch.add_primitive(point.clone());
            sketch.add_primitive(arc.clone());

            let constraint = Rc::new(RefCell::new(ArcEndPointCoincident::new(
                arc.clone(),
                point.clone(),
            )));
            sketch.add_constraint(constraint.clone());
            sketch.add_constraint(constraint.clone());
        })
        .is_err());
    }
}
