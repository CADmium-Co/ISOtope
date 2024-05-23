pub mod start_end_connected;

pub trait Constraint {
    fn loss_value(&self) -> f64;
    fn gradient(&self) -> Vec<f64>;
}
