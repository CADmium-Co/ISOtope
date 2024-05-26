use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use nalgebra::{DVector, DVectorView};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod arc;
pub mod circle;
pub mod line;
pub mod point2;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a fixed number of parameters that can be used for gradient descent.
pub trait Parametric: Debug {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>>;
    fn zero_gradient(&mut self);
    fn get_data(&self) -> DVector<f64>;
    fn set_data(&mut self, data: DVectorView<f64>);
    fn get_gradient(&self) -> DVector<f64>;
    fn to_primitive(&self) -> Primitive;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Primitive {
    Point2(point2::Point2),
    Line(line::Line),
    Arc(arc::Arc),
    Circle(circle::Circle),
}

impl Parametric for Primitive {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>> {
        match self {
            Primitive::Point2(p) => p.references(),
            Primitive::Line(l) => l.references(),
            Primitive::Arc(a) => a.references(),
            Primitive::Circle(c) => c.references(),
        }
    }

    fn zero_gradient(&mut self) {
        match self {
            Primitive::Point2(p) => p.zero_gradient(),
            Primitive::Line(l) => l.zero_gradient(),
            Primitive::Arc(a) => a.zero_gradient(),
            Primitive::Circle(c) => c.zero_gradient(),
        }
    }

    fn get_data(&self) -> DVector<f64> {
        match self {
            Primitive::Point2(p) => p.get_data(),
            Primitive::Line(l) => l.get_data(),
            Primitive::Arc(a) => a.get_data(),
            Primitive::Circle(c) => c.get_data(),
        }
    }

    fn set_data(&mut self, data: DVectorView<f64>) {
        match self {
            Primitive::Point2(p) => p.set_data(data),
            Primitive::Line(l) => l.set_data(data),
            Primitive::Arc(a) => a.set_data(data),
            Primitive::Circle(c) => c.set_data(data),
        }
    }

    fn get_gradient(&self) -> DVector<f64> {
        match self {
            Primitive::Point2(p) => p.get_gradient(),
            Primitive::Line(l) => l.get_gradient(),
            Primitive::Arc(a) => a.get_gradient(),
            Primitive::Circle(c) => c.get_gradient(),
        }
    }

    fn to_primitive(&self) -> Primitive {
        self.clone()
    }
}

// Custom serialization for VecDeque<Rc<RefCell<dyn Parametric>>>
pub fn serialize_primitives<S>(
    primitives: &VecDeque<Rc<RefCell<dyn Parametric>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(primitives.len()))?;
    for primitive in primitives {
        seq.serialize_element(&primitive.borrow().to_primitive())?;
    }
    seq.end()
}

// Custom deserialization for VecDeque<Rc<RefCell<dyn Parametric>>>
pub fn deserialize_primitives<'de, D>(
    deserializer: D,
) -> Result<VecDeque<Rc<RefCell<dyn Parametric>>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PrimitiveVisitor;

    impl<'de> Visitor<'de> for PrimitiveVisitor {
        type Value = VecDeque<Rc<RefCell<dyn Parametric>>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of primitives")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut primitives = VecDeque::new();
            while let Some(primitive) = seq.next_element::<Primitive>()? {
                let parametric: Rc<RefCell<dyn Parametric>> = match primitive {
                    // TODO: Macro this
                    Primitive::Arc(inner) => Rc::new(RefCell::new(inner)),
                    Primitive::Circle(inner) => Rc::new(RefCell::new(inner)),
                    Primitive::Line(inner) => Rc::new(RefCell::new(inner)),
                    Primitive::Point2(inner) => Rc::new(RefCell::new(inner)),
                };
                primitives.push_back(parametric);
            }
            Ok(primitives)
        }
    }

    deserializer.deserialize_seq(PrimitiveVisitor)
}
