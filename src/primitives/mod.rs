use std::cell::{Ref, RefCell, RefMut};
use std::fmt::Debug;
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
pub trait Parametric: Debug {
    fn references(&self) -> Vec<ParametricCell>;
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
    fn references(&self) -> Vec<ParametricCell> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "tsify", derive(Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ParametricCell {
    Point2(Rc<RefCell<point2::Point2>>),
    Line(Rc<RefCell<line::Line>>),
    Arc(Rc<RefCell<arc::Arc>>),
    Circle(Rc<RefCell<circle::Circle>>),
}

impl ParametricCell {
    pub fn borrow(&self) -> Ref<dyn Parametric> {
        match self {
            ParametricCell::Point2(p) => p.borrow(),
            ParametricCell::Line(l) => l.borrow(),
            ParametricCell::Arc(a) => a.borrow(),
            ParametricCell::Circle(c) => c.borrow(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<dyn Parametric> {
        match self {
            ParametricCell::Point2(p) => p.borrow_mut(),
            ParametricCell::Line(l) => l.borrow_mut(),
            ParametricCell::Arc(a) => a.borrow_mut(),
            ParametricCell::Circle(c) => c.borrow_mut(),
        }
    }

    pub fn ptr_eq(&self, other: &ParametricCell) -> bool {
        match (self, other) {
            (ParametricCell::Point2(p1), ParametricCell::Point2(p2)) => Rc::ptr_eq(p1, p2),
            (ParametricCell::Line(l1), ParametricCell::Line(l2)) => Rc::ptr_eq(l1, l2),
            (ParametricCell::Arc(a1), ParametricCell::Arc(a2)) => Rc::ptr_eq(a1, a2),
            (ParametricCell::Circle(c1), ParametricCell::Circle(c2)) => Rc::ptr_eq(c1, c2),
            _ => false,
        }
    }
}

impl Parametric for ParametricCell {
    fn references(&self) -> Vec<ParametricCell> {
        match self {
            ParametricCell::Point2(p) => p.borrow().references(),
            ParametricCell::Line(l) => l.borrow().references(),
            ParametricCell::Arc(a) => a.borrow().references(),
            ParametricCell::Circle(c) => c.borrow().references(),
        }
    }

    fn zero_gradient(&mut self) {
        match self {
            ParametricCell::Point2(p) => p.borrow_mut().zero_gradient(),
            ParametricCell::Line(l) => l.borrow_mut().zero_gradient(),
            ParametricCell::Arc(a) => a.borrow_mut().zero_gradient(),
            ParametricCell::Circle(c) => c.borrow_mut().zero_gradient(),
        }
    }

    fn get_data(&self) -> DVector<f64> {
        match self {
            ParametricCell::Point2(p) => p.borrow().get_data(),
            ParametricCell::Line(l) => l.borrow().get_data(),
            ParametricCell::Arc(a) => a.borrow().get_data(),
            ParametricCell::Circle(c) => c.borrow().get_data(),
        }
    }

    fn set_data(&mut self, data: DVectorView<f64>) {
        match self {
            ParametricCell::Point2(p) => p.borrow_mut().set_data(data),
            ParametricCell::Line(l) => l.borrow_mut().set_data(data),
            ParametricCell::Arc(a) => a.borrow_mut().set_data(data),
            ParametricCell::Circle(c) => c.borrow_mut().set_data(data),
        }
    }

    fn get_gradient(&self) -> DVector<f64> {
        match self {
            ParametricCell::Point2(p) => p.borrow().get_gradient(),
            ParametricCell::Line(l) => l.borrow().get_gradient(),
            ParametricCell::Arc(a) => a.borrow().get_gradient(),
            ParametricCell::Circle(c) => c.borrow().get_gradient(),
        }
    }

    fn to_primitive(&self) -> Primitive {
        match self {
            ParametricCell::Point2(p) => Primitive::Point2(p.borrow().clone()),
            ParametricCell::Line(l) => Primitive::Line(l.borrow().clone()),
            ParametricCell::Arc(a) => Primitive::Arc(a.borrow().clone()),
            ParametricCell::Circle(c) => Primitive::Circle(c.borrow().clone()),
        }
    }
}

impl PartialEq for ParametricCell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ParametricCell::Point2(p1), ParametricCell::Point2(p2)) => Rc::ptr_eq(p1, p2),
            (ParametricCell::Line(l1), ParametricCell::Line(l2)) => Rc::ptr_eq(l1, l2),
            (ParametricCell::Arc(a1), ParametricCell::Arc(a2)) => Rc::ptr_eq(a1, a2),
            (ParametricCell::Circle(c1), ParametricCell::Circle(c2)) => Rc::ptr_eq(c1, c2),
            _ => false,
        }
    }
}
