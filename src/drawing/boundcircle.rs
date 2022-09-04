use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, ShapePath};
use bevy_prototype_lyon::shapes::Circle;

use crate::variables::binding::Bound;

use super::boundpoint::BoundPoint;

#[derive(Component, Clone)]
pub struct BoundCircle {
    radius: Entity,
    pub radius_value: f32,
}

impl BoundCircle {
    pub fn new(radius: Entity) -> Self {
        Self {
            radius,
            radius_value: 1.,
        }
    }
}

pub(crate) fn update_bound_circles(
    mut circle_query: Query<(&mut Path, &mut Transform, &BoundCircle, &BoundPoint)>,
) {
    for (mut path, mut transform, circle, point) in circle_query.iter_mut() {
        let circle = Circle {
            radius: circle.radius_value as f32,
            ..Circle::default()
        };
        *path = ShapePath::build_as(&circle);
        let vec: Vec2 = point.vec2();
        *transform = Transform::from_xyz(vec.x, vec.y, 0.);
    }
}

impl Bound for BoundCircle {
    fn get_bindings(&self) -> Vec<Entity> {
        vec![self.radius]
    }

    fn set_bindings(&mut self, mut bindings: Vec<f64>) {
        self.radius_value = bindings.pop().unwrap() as f32;
    }
}
