use std::rc::Rc;

use crate::sketch::{constraints::Constraint, primitives::line::Line};

// This is a sketch constraint that connects the end of the first line to the start of the following line.
pub struct LineLineStartEndConnected<'a> {
    pub first_line: Rc<Line<'a>>,
    pub following_line: Rc<Line<'a>>,
}

impl<'a> LineLineStartEndConnected<'a> {
    pub fn new(first_line: Rc<Line<'a>>, following_line: Rc<Line<'a>>) -> Self {
        Self {
            first_line,
            following_line,
        }
    }
}

impl Constraint for LineLineStartEndConnected<'_> {
    fn loss_value(&self) -> f64 {
        let first_line_end = self.first_line.end();
        let following_line_start = &self.following_line.start();

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        dx * dx + dy * dy
    }

    fn gradient(&self) -> Vec<f64> {
        let first_line_end = self.first_line.end();
        let following_line_start = self.following_line.start();

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

