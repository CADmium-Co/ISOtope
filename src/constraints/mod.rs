use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

// Custom serialization for VecDeque<Rc<RefCell<dyn Constraint>>>
pub fn serialize_constraints<S>(
    constraints: &VecDeque<Rc<RefCell<dyn Constraint>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(constraints.len()))?;
    for primitive in constraints {
        seq.serialize_element(&primitive.borrow().get_type())?;
    }
    seq.end()
}

// Custom deserialization for VecDeque<Rc<RefCell<dyn Constraint>>>
pub fn deserialize_constraints<'de, D>(
    deserializer: D,
) -> Result<VecDeque<Rc<RefCell<dyn Constraint>>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ConstraintVisitor;

    impl<'de> Visitor<'de> for ConstraintVisitor {
        type Value = VecDeque<Rc<RefCell<dyn Constraint>>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of constraints")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut constraints = VecDeque::new();
            while let Some(constraint) = seq.next_element::<ConstraintType>()? {
                let constraint: Rc<RefCell<dyn Constraint>> = match constraint {
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
                constraints.push_back(constraint);
            }
            Ok(constraints)
        }
    }

    deserializer.deserialize_seq(ConstraintVisitor)
}
