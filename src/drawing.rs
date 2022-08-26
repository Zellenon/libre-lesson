use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder, ShapePath};
use bevy_prototype_lyon::shapes::Circle;

use crate::variables::OldVariable;

pub struct DrawingPlugin;
impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bound_lines);
    }
}

#[derive(Component, Copy, Clone)]
struct BoundPoint {
    x: OldVariable<f64>,
    y: OldVariable<f64>,
}

impl BoundPoint {
    pub fn vec2(self) -> Vec2 {
        Vec2::new(self.x.0 as f32, self.y.0 as f32)
    }
}

#[derive(Component, Copy, Clone)]
struct BoundLine {
    p1: BoundPoint,
    p2: BoundPoint,
}

impl BoundLine {
    pub fn new(p1: BoundPoint, p2: BoundPoint) -> Self {
        Self { p1, p2 }
    }
}

fn update_bound_lines(mut line_query: Query<(&mut BoundLine, &mut Path)>) {
    for (line, mut path) in line_query.iter_mut() {
        let mut path_builder = PathBuilder::new();
        path_builder.line_to(line.p1.vec2());
        path_builder.line_to(line.p2.vec2());
        *path = path_builder.build();
    }
}

#[derive(Component, Copy, Clone)]
pub struct BoundCircle {
    radius: f64,
}

fn update_bound_circles(
    mut circle_query: Query<(&mut Path, &mut Transform, &BoundCircle, &BoundPoint)>,
) {
    for (mut path, mut transform, circle, point) in circle_query.iter_mut() {
        let circle = Circle {
            radius: circle.radius as f32,
            ..Circle::default()
        };
        // let mut new_path = circle_query.single_mut();
        *path = ShapePath::build_as(&circle);
        *transform = Transform::from_xyz(point.x.0 as f32, point.y.0 as f32, 0.);
    }
}
