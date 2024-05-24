use std::{cell::RefCell, rc::Rc};

use crate::sketch::{constraints::Constraint, primitives::line::Line};

// This is a sketch constraint that connects the end of the first line to the start of the following line.
pub struct LineLineStartEndConnected {
    pub first_line: Rc<RefCell<Line>>,
    pub following_line: Rc<RefCell<Line>>,
}

impl LineLineStartEndConnected {
    pub fn new(first_line: Rc<RefCell<Line>>, following_line: Rc<RefCell<Line>>) -> Self {
        Self {
            first_line,
            following_line,
        }
    }
}

impl Constraint for LineLineStartEndConnected {
    fn loss_value(&self) -> f64 {
        let first_line_end = self.first_line.borrow().end();
        let following_line_start = &self.following_line.borrow().start();

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        dx * dx + dy * dy
    }

    fn update_gradient(&mut self) {
        let first_line_end = self.first_line.borrow().end();
        let following_line_start = self.following_line.borrow().start();

        let dx = first_line_end.x - following_line_start.x;
        let dy = first_line_end.y - following_line_start.y;

        self.first_line
            .as_ref()
            .borrow_mut()
            .add_to_gradient(0.0, 0.0, dx, dy);
        self.following_line
            .as_ref()
            .borrow_mut()
            .add_to_gradient(-dx, -dy, 0.0, 0.0);
    }
}
