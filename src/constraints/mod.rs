pub mod coincident;
pub mod lines;
pub mod angle_between_points;

pub trait Constraint {
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
}
