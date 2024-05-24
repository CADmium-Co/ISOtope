use nalgebra::Vector2;

use super::Parametric;

pub struct Circle {
    data: [f64; 3],
    gradient: [f64; 3],
}

impl Circle {
    pub fn new(center: Vector2<f64>, radius: f64) -> Self {
        Self {
            data: [center.x, center.y, radius],
            gradient: [0.0; 3],
        }
    }

    pub fn center(&self) -> Vector2<f64> {
        Vector2::new(self.data[0], self.data[1])   
    }

    pub fn radius(&self) -> f64 {
        self.data[2]
    }

    pub fn set_center(&mut self, center: Vector2<f64>) {
        self.data[0] = center.x;
        self.data[1] = center.y;
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.data[2] = radius;
    }

    pub fn add_to_gradient(
        &mut self,
        gradient_center_x: f64,
        gradient_center_y: f64,
        gradient_radius: f64,
    ) {
        self.gradient[0] += gradient_center_x;
        self.gradient[1] += gradient_center_y;
        self.gradient[2] += gradient_radius;
    }
}

impl Parametric for Circle {
    fn references(&self) -> Vec<std::rc::Rc<std::cell::RefCell<dyn Parametric>>> {
        vec![]
    }

    fn zero_gradient(&mut self) {
        self.gradient = [0.0; 3];
    }

    fn step(&mut self, step_size: f64) {
        for i in 0..3 {
            self.data[i] -= step_size * self.gradient[i];
        }
    }
}
