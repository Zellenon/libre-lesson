use bevy::prelude::*;

use crate::variables::binding::Bound;

#[derive(Component, Clone)]
pub struct BoundPoint {
    x_value: f32,
    x: Entity,
    y_value: f32,
    y: Entity,
}

impl BoundPoint {
    pub fn new(x: Entity, y: Entity) -> Self {
        Self {
            x,
            y,
            x_value: 1.,
            y_value: 1.,
        }
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x_value, self.y_value)
    }
}

impl Bound for BoundPoint {
    fn get_bindings(&self) -> Vec<Entity> {
        vec![self.x, self.y]
    }

    fn set_bindings(&mut self, mut bindings: Vec<f64>) {
        self.y_value = bindings.pop().unwrap() as f32;
        self.x_value = bindings.pop().unwrap() as f32;
    }
}
