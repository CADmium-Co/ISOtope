use std::{cell::RefCell, rc::Rc};

use nalgebra::{DVector, DVectorView};

pub mod arc;
pub mod circle;
pub mod line;
pub mod point2;

// A trait that defines a parametric object, meaning a SketchPrimitive that can be defined by a fixed number of parameters that can be used for gradient descent.
pub trait Parametric {
    fn references(&self) -> Vec<Rc<RefCell<dyn Parametric>>>;
    fn zero_gradient(&mut self);
    fn get_data(&self) -> DVector<f64>;
    fn set_data(&mut self, data: DVectorView<f64>);
    fn get_gradient(&self) -> DVector<f64>;
    fn to_primitive(&self) -> Primitive;
}

#[derive(Debug, Clone)]
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
