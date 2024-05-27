use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
use std::ptr;
use std::rc::Rc;

use nalgebra::{DVector, DVectorView};
use serde::{Deserialize, Serialize};

#[cfg(feature = "tsify")]
use tsify::Tsify;

pub mod arc;
pub mod circle;
pub mod line;
pub mod point2;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a fixed number of parameters that can be used for gradient descent.
pub trait PrimitiveLike: Debug {
    fn references(&self) -> Vec<PrimitiveCell>;
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

impl Primitive {
    pub fn as_primitive_like(&self) -> &dyn PrimitiveLike {
        match self {
            Primitive::Point2(p) => p,
            Primitive::Line(l) => l,
            Primitive::Arc(a) => a,
            Primitive::Circle(c) => c,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum PrimitiveCell {
    Point2(Rc<RefCell<point2::Point2>>),
    Line(Rc<RefCell<line::Line>>),
    Arc(Rc<RefCell<arc::Arc>>),
    Circle(Rc<RefCell<circle::Circle>>),
}

impl PrimitiveCell {
    pub fn borrow(&self) -> Ref<dyn PrimitiveLike> {
        match self {
            PrimitiveCell::Point2(p) => p.borrow(),
            PrimitiveCell::Line(l) => l.borrow(),
            PrimitiveCell::Arc(a) => a.borrow(),
            PrimitiveCell::Circle(c) => c.borrow(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<dyn PrimitiveLike> {
        match self {
            PrimitiveCell::Point2(p) => p.borrow_mut(),
            PrimitiveCell::Line(l) => l.borrow_mut(),
            PrimitiveCell::Arc(a) => a.borrow_mut(),
            PrimitiveCell::Circle(c) => c.borrow_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const dyn PrimitiveLike {
        match self {
            PrimitiveCell::Point2(p) => p.as_ptr(),
            PrimitiveCell::Line(l) => l.as_ptr(),
            PrimitiveCell::Arc(a) => a.as_ptr(),
            PrimitiveCell::Circle(c) => c.as_ptr(),
        }
    }
}

impl PartialEq for PrimitiveCell {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.as_ptr(), other.as_ptr())
    }
}
