pub mod start_end_connected;

pub trait Constraint {
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
}
