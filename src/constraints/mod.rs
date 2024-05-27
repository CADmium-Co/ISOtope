use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::ptr;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::primitives::ParametricCell;

pub mod angle_between_points;
pub mod coincident;
pub mod distance;
pub mod fix_point;
pub mod lines;

pub trait Constraint: Debug {
    fn references(&self) -> Vec<ParametricCell>;
    fn loss_value(&self) -> f64;
    fn update_gradient(&mut self);
    fn get_type(&self) -> ConstraintType;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ConstraintType {
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

impl Constraint for ConstraintType {
    fn references(&self) -> Vec<ParametricCell> {
        match self {
            ConstraintType::AngleBetweenPoints(c) => c.references(),
            ConstraintType::ArcEndPointCoincident(c) => c.references(),
            ConstraintType::ArcStartPointCoincident(c) => c.references(),
            ConstraintType::EuclideanDistance(c) => c.references(),
            ConstraintType::HorizontalDistance(c) => c.references(),
            ConstraintType::VerticalDistance(c) => c.references(),
            ConstraintType::FixPoint(c) => c.references(),
            ConstraintType::EqualLength(c) => c.references(),
            ConstraintType::HorizontalLine(c) => c.references(),
            ConstraintType::VerticalLine(c) => c.references(),
            ConstraintType::ParallelLines(c) => c.references(),
            ConstraintType::PerpendicularLines(c) => c.references(),
        }
    }

    fn loss_value(&self) -> f64 {
        match self {
            ConstraintType::AngleBetweenPoints(c) => c.loss_value(),
            ConstraintType::ArcEndPointCoincident(c) => c.loss_value(),
            ConstraintType::ArcStartPointCoincident(c) => c.loss_value(),
            ConstraintType::EuclideanDistance(c) => c.loss_value(),
            ConstraintType::HorizontalDistance(c) => c.loss_value(),
            ConstraintType::VerticalDistance(c) => c.loss_value(),
            ConstraintType::FixPoint(c) => c.loss_value(),
            ConstraintType::EqualLength(c) => c.loss_value(),
            ConstraintType::HorizontalLine(c) => c.loss_value(),
            ConstraintType::VerticalLine(c) => c.loss_value(),
            ConstraintType::ParallelLines(c) => c.loss_value(),
            ConstraintType::PerpendicularLines(c) => c.loss_value(),
        }
    }

    fn update_gradient(&mut self) {
        match self {
            ConstraintType::AngleBetweenPoints(c) => c.update_gradient(),
            ConstraintType::ArcEndPointCoincident(c) => c.update_gradient(),
            ConstraintType::ArcStartPointCoincident(c) => c.update_gradient(),
            ConstraintType::EuclideanDistance(c) => c.update_gradient(),
            ConstraintType::HorizontalDistance(c) => c.update_gradient(),
            ConstraintType::VerticalDistance(c) => c.update_gradient(),
            ConstraintType::FixPoint(c) => c.update_gradient(),
            ConstraintType::EqualLength(c) => c.update_gradient(),
            ConstraintType::HorizontalLine(c) => c.update_gradient(),
            ConstraintType::VerticalLine(c) => c.update_gradient(),
            ConstraintType::ParallelLines(c) => c.update_gradient(),
            ConstraintType::PerpendicularLines(c) => c.update_gradient(),
        }
    }

    fn get_type(&self) -> ConstraintType {
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
    pub fn borrow(&self) -> Ref<dyn Constraint> {
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

    pub fn borrow_mut(&self) -> RefMut<dyn Constraint> {
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

    pub fn as_ptr(&self) -> *const dyn Constraint {
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
