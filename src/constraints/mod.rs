use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "tsify")]
use tsify::Tsify;

use crate::primitives::Parametric;

pub mod angle_between_points;
pub mod coincident;
pub mod distance;
pub mod fix_point;
pub mod lines;

pub trait Constraint: Debug {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>>;
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

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ConstraintCell(pub Rc<RefCell<dyn Constraint>>);

impl Serialize for ConstraintCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().get_type().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for ConstraintCell {
    fn deserialize<D>(deserializer: D) -> Result<ConstraintCell, D::Error>
    where
        D: Deserializer<'a>,
    {
        let constraint_type = ConstraintType::deserialize(deserializer)?;
        let constraint: Rc<RefCell<dyn Constraint>> = match constraint_type {
            // TODO: Macro this
            ConstraintType::AngleBetweenPoints(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::ArcEndPointCoincident(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::ArcStartPointCoincident(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::EuclideanDistance(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::HorizontalDistance(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::VerticalDistance(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::FixPoint(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::EqualLength(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::HorizontalLine(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::VerticalLine(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::ParallelLines(inner) => Rc::new(RefCell::new(inner)),
            ConstraintType::PerpendicularLines(inner) => Rc::new(RefCell::new(inner)),
        };

        Ok(ConstraintCell(constraint))
    }
}
