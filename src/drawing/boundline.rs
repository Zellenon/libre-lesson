use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder};

use crate::variables::binding::Bound;

#[derive(Component, Clone)]
pub struct BoundLine {
    x1: Entity,
    x2: Entity,
    y1: Entity,
    y2: Entity,
    x1_value: f32,
    x2_value: f32,
    y1_value: f32,
    y2_value: f32,
}

impl BoundLine {
    pub fn new(x1: Entity, y1: Entity, x2: Entity, y2: Entity) -> Self {
        Self {
            x1,
            x2,
            y1,
            y2,
            x1_value: 1.,
            x2_value: 1.,
            y1_value: 1.,
            y2_value: 1.,
        }
    }
}

pub(crate) fn update_bound_lines(mut line_query: Query<(&mut BoundLine, &mut Path)>) {
    for (line, mut path) in line_query.iter_mut() {
        let mut path_builder = PathBuilder::new();
        path_builder.line_to(Vec2::new(line.x1_value, line.y1_value));
        path_builder.line_to(Vec2::new(line.x2_value, line.y2_value));
        *path = path_builder.build();
    }
}

impl Bound for BoundLine {
    fn get_bindings(&self) -> Vec<Entity> {
        vec![self.x1, self.x2, self.y1, self.y2]
    }

    fn set_bindings(&mut self, mut bindings: Vec<f64>) {
        self.y2_value = bindings.pop().unwrap() as f32;
        self.y1_value = bindings.pop().unwrap() as f32;
        self.x2_value = bindings.pop().unwrap() as f32;
        self.x1_value = bindings.pop().unwrap() as f32;
    }
}
