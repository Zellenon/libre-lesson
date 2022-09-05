use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder};

use crate::variables::binding::Bound;

#[derive(Component, Clone)]
pub struct BoundTracker {
    target: Entity,
    pub target_value: f32,
    pub history: Vec<f32>,
    pub max_length: usize,
}
impl BoundTracker {
    pub fn new(target: Entity, max_length: usize) -> Self {
        Self {
            target,
            history: Vec::new(),
            target_value: 1.,
            max_length,
        }
    }
}

pub(crate) fn update_bound_trackers(mut tracker_query: Query<(&mut Path, &mut BoundTracker)>) {
    for (mut line, mut tracker) in &mut tracker_query.iter_mut() {
        let mut path_builder = PathBuilder::new();

        let new_y = tracker.target_value;
        tracker.history.insert(0, new_y);
        if tracker.history.len() > tracker.max_length {
            tracker.history.pop();
        }

        for (index, vertex) in tracker.history.iter().enumerate() {
            path_builder.line_to(Vec2::new(index as f32 * 2., *vertex as f32));
        }
        let new_path = path_builder.build();
        *line = new_path;
    }
}

impl Bound for BoundTracker {
    fn get_bindings(&self) -> Vec<Entity> {
        vec![self.target]
    }

    fn set_bindings(&mut self, mut bindings: Vec<f64>) {
        self.target_value = bindings.pop().unwrap() as f32;
    }
}
