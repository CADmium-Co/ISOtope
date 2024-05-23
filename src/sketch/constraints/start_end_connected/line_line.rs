use std::rc::Rc;

use crate::sketch::{constraints::Constraint, primitives::line::Line};

// This is a sketch constraint that connects the end of the first line to the start of the following line.
pub struct LineLineStartEndConnected {
    pub first_line: Rc<Line>,
    pub following_line: Rc<Line>,
}

impl LineLineStartEndConnected {
    pub fn new(first_line: Rc<Line>, following_line: Rc<Line>) -> Self {
        Self {
            first_line,
            following_line,
        }
    }
}

impl Constraint for LineLineStartEndConnected {
    fn loss_value(&self) -> f64 {
        let first_line_end = &self.first_line.end;
        let following_line_start = &self.following_line.start;

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        dx * dx + dy * dy
    }

    fn gradient(&self) -> Vec<f64> {
        let first_line_end = &self.first_line.end;
        let following_line_start = &self.following_line.start;

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        vec![
            2.0 * dx,
            2.0 * dy,
            -2.0 * dx,
            -2.0 * dy,
        ]
    }
}

