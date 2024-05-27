use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use nalgebra::{DVector, DVectorView};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "tsify")]
use tsify::Tsify;

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
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
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

#[repr(transparent)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ParametricCell(pub Rc<RefCell<dyn Parametric>>);

impl Into<Primitive> for ParametricCell {
    fn into(self) -> Primitive {
        self.0.borrow().to_primitive()
    }
}

impl Into<ParametricCell> for Primitive {
    fn into(self) -> ParametricCell {
        ParametricCell(Rc::new(RefCell::new(self)))
    }
}

impl Serialize for ParametricCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().to_primitive().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for ParametricCell {
    fn deserialize<D>(deserializer: D) -> Result<ParametricCell, D::Error>
    where
        D: Deserializer<'a>,
    {
        let primitive = Primitive::deserialize(deserializer)?;
        let parametric: Rc<RefCell<dyn Parametric>> = match primitive {
            // TODO: Macro this
            Primitive::Arc(inner) => Rc::new(RefCell::new(inner)),
            Primitive::Circle(inner) => Rc::new(RefCell::new(inner)),
            Primitive::Line(inner) => Rc::new(RefCell::new(inner)),
            Primitive::Point2(inner) => Rc::new(RefCell::new(inner)),
        };
        Ok(ParametricCell(parametric))
    }
}
