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

    pub fn as_ptr(&self) -> *const dyn Parametric {
        match self {
            ParametricCell::Point2(p) => p.as_ptr(),
            ParametricCell::Line(l) => l.as_ptr(),
            ParametricCell::Arc(a) => a.as_ptr(),
            ParametricCell::Circle(c) => c.as_ptr(),
        }
    }
}

// impl Parametric for ParametricCell {
//     fn references(&self) -> Vec<ParametricCell> {
//         self.borrow().references()
//     }

//     fn zero_gradient(&mut self) {
//         self.borrow_mut().zero_gradient();
//     }

//     fn get_data(&self) -> DVector<f64> {
//         self.borrow().get_data()
//     }

//     fn set_data(&mut self, data: DVectorView<f64>) {
//         self.borrow_mut().set_data(data);
//     }

//     fn get_gradient(&self) -> DVector<f64> {
//         self.borrow().get_gradient()
//     }

//     fn to_primitive(&self) -> Primitive {
//         self.borrow().to_primitive()
//     }
// }

impl PartialEq for ParametricCell {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.as_ptr(), other.as_ptr())
    }
}
