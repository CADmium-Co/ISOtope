pub mod start_end_connected;

pub trait Constraint {
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
}

pub enum Constraints<'a> {
    StartEndConnected(start_end_connected::line_line::LineLineStartEndConnected<'a>),
}

impl<'a> Constraints<'a> {
    pub fn constraint(&'a self) -> &'a dyn Constraint {
        match self {
            Constraints::StartEndConnected(c) => c,
        }
    }
    pub fn constraint_mut(&'a mut self) -> &'a mut dyn Constraint {
        match self {
            Constraints::StartEndConnected(c) => c,
        }
    }
}