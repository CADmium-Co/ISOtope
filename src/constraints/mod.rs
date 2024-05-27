use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::ptr;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::primitives::PrimitiveCell;

pub mod angle_between_points;
pub mod coincident;
pub mod distance;
pub mod fix_point;
pub mod lines;

pub trait ConstraintLike: Debug {
    fn references(&self) -> Vec<PrimitiveCell>;
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
    fn get_type(&self) -> Constraint;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Constraint {
    AngleBetweenPoints(angle_between_points::AngleBetweenPoints),
    ArcEndPointCoincident(coincident::arc_end_point_coincident::ArcEndPointCoincident),
    ArcStartPointCoincident(coincident::arc_start_point_coincident::ArcStartPointCoincident),
    EuclideanDistance(distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints),
    HorizontalDistance(
        distance::horizontal_distance_between_points::HorizontalDistanceBetweenPoints,
    ),
    VerticalDistance(distance::vertical_distance_between_points::VerticalDistanceBetweenPoints),
    FixPoint(fix_point::FixPoint),
    EqualLength(lines::equal_length::EqualLength),
    HorizontalLine(lines::horizontal_line::HorizontalLine),
    VerticalLine(lines::vertical_line::VerticalLine),
    ParallelLines(lines::parallel_lines::ParallelLines),
    PerpendicularLines(lines::perpendicular_lines::PerpendicularLines),
}

impl ConstraintLike for Constraint {
    fn references(&self) -> Vec<PrimitiveCell> {
        match self {
            Constraint::AngleBetweenPoints(c) => c.references(),
            Constraint::ArcEndPointCoincident(c) => c.references(),
            Constraint::ArcStartPointCoincident(c) => c.references(),
            Constraint::EuclideanDistance(c) => c.references(),
            Constraint::HorizontalDistance(c) => c.references(),
            Constraint::VerticalDistance(c) => c.references(),
            Constraint::FixPoint(c) => c.references(),
            Constraint::EqualLength(c) => c.references(),
            Constraint::HorizontalLine(c) => c.references(),
            Constraint::VerticalLine(c) => c.references(),
            Constraint::ParallelLines(c) => c.references(),
            Constraint::PerpendicularLines(c) => c.references(),
        }
    }

    fn loss_value(&self) -> f64 {
        match self {
            Constraint::AngleBetweenPoints(c) => c.loss_value(),
            Constraint::ArcEndPointCoincident(c) => c.loss_value(),
            Constraint::ArcStartPointCoincident(c) => c.loss_value(),
            Constraint::EuclideanDistance(c) => c.loss_value(),
            Constraint::HorizontalDistance(c) => c.loss_value(),
            Constraint::VerticalDistance(c) => c.loss_value(),
            Constraint::FixPoint(c) => c.loss_value(),
            Constraint::EqualLength(c) => c.loss_value(),
            Constraint::HorizontalLine(c) => c.loss_value(),
            Constraint::VerticalLine(c) => c.loss_value(),
            Constraint::ParallelLines(c) => c.loss_value(),
            Constraint::PerpendicularLines(c) => c.loss_value(),
        }
    }

    fn update_gradient(&mut self) {
        match self {
            Constraint::AngleBetweenPoints(c) => c.update_gradient(),
            Constraint::ArcEndPointCoincident(c) => c.update_gradient(),
            Constraint::ArcStartPointCoincident(c) => c.update_gradient(),
            Constraint::EuclideanDistance(c) => c.update_gradient(),
            Constraint::HorizontalDistance(c) => c.update_gradient(),
            Constraint::VerticalDistance(c) => c.update_gradient(),
            Constraint::FixPoint(c) => c.update_gradient(),
            Constraint::EqualLength(c) => c.update_gradient(),
            Constraint::HorizontalLine(c) => c.update_gradient(),
            Constraint::VerticalLine(c) => c.update_gradient(),
            Constraint::ParallelLines(c) => c.update_gradient(),
            Constraint::PerpendicularLines(c) => c.update_gradient(),
        }
    }

    fn get_type(&self) -> Constraint {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ConstraintCell {
    AngleBetweenPoints(Rc<RefCell<angle_between_points::AngleBetweenPoints>>),
    ArcEndPointCoincident(Rc<RefCell<coincident::arc_end_point_coincident::ArcEndPointCoincident>>),
    ArcStartPointCoincident(
        Rc<RefCell<coincident::arc_start_point_coincident::ArcStartPointCoincident>>,
    ),
    EuclideanDistance(
        Rc<RefCell<distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints>>,
    ),
    HorizontalDistance(
        Rc<RefCell<distance::horizontal_distance_between_points::HorizontalDistanceBetweenPoints>>,
    ),
    VerticalDistance(
        Rc<RefCell<distance::vertical_distance_between_points::VerticalDistanceBetweenPoints>>,
    ),
    FixPoint(Rc<RefCell<fix_point::FixPoint>>),
    EqualLength(Rc<RefCell<lines::equal_length::EqualLength>>),
    HorizontalLine(Rc<RefCell<lines::horizontal_line::HorizontalLine>>),
    VerticalLine(Rc<RefCell<lines::vertical_line::VerticalLine>>),
    ParallelLines(Rc<RefCell<lines::parallel_lines::ParallelLines>>),
    PerpendicularLines(Rc<RefCell<lines::perpendicular_lines::PerpendicularLines>>),
}

impl ConstraintCell {
    pub fn borrow(&self) -> Ref<dyn ConstraintLike> {
        match self {
            ConstraintCell::AngleBetweenPoints(c) => c.borrow(),
            ConstraintCell::ArcEndPointCoincident(c) => c.borrow(),
            ConstraintCell::ArcStartPointCoincident(c) => c.borrow(),
            ConstraintCell::EuclideanDistance(c) => c.borrow(),
            ConstraintCell::HorizontalDistance(c) => c.borrow(),
            ConstraintCell::VerticalDistance(c) => c.borrow(),
            ConstraintCell::FixPoint(c) => c.borrow(),
            ConstraintCell::EqualLength(c) => c.borrow(),
            ConstraintCell::HorizontalLine(c) => c.borrow(),
            ConstraintCell::VerticalLine(c) => c.borrow(),
            ConstraintCell::ParallelLines(c) => c.borrow(),
            ConstraintCell::PerpendicularLines(c) => c.borrow(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<dyn ConstraintLike> {
        match self {
            ConstraintCell::AngleBetweenPoints(c) => c.borrow_mut(),
            ConstraintCell::ArcEndPointCoincident(c) => c.borrow_mut(),
            ConstraintCell::ArcStartPointCoincident(c) => c.borrow_mut(),
            ConstraintCell::EuclideanDistance(c) => c.borrow_mut(),
            ConstraintCell::HorizontalDistance(c) => c.borrow_mut(),
            ConstraintCell::VerticalDistance(c) => c.borrow_mut(),
            ConstraintCell::FixPoint(c) => c.borrow_mut(),
            ConstraintCell::EqualLength(c) => c.borrow_mut(),
            ConstraintCell::HorizontalLine(c) => c.borrow_mut(),
            ConstraintCell::VerticalLine(c) => c.borrow_mut(),
            ConstraintCell::ParallelLines(c) => c.borrow_mut(),
            ConstraintCell::PerpendicularLines(c) => c.borrow_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const dyn ConstraintLike {
        match self {
            ConstraintCell::AngleBetweenPoints(c) => c.as_ptr(),
            ConstraintCell::ArcEndPointCoincident(c) => c.as_ptr(),
            ConstraintCell::ArcStartPointCoincident(c) => c.as_ptr(),
            ConstraintCell::EuclideanDistance(c) => c.as_ptr(),
            ConstraintCell::HorizontalDistance(c) => c.as_ptr(),
            ConstraintCell::VerticalDistance(c) => c.as_ptr(),
            ConstraintCell::FixPoint(c) => c.as_ptr(),
            ConstraintCell::EqualLength(c) => c.as_ptr(),
            ConstraintCell::HorizontalLine(c) => c.as_ptr(),
            ConstraintCell::VerticalLine(c) => c.as_ptr(),
            ConstraintCell::ParallelLines(c) => c.as_ptr(),
            ConstraintCell::PerpendicularLines(c) => c.as_ptr(),
        }
    }
}

impl PartialEq for ConstraintCell {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.as_ptr(), other.as_ptr())
    }
}
