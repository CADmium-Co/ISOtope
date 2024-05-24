
pub mod coincident;

pub trait Constraint {
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
}
