use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder, ShapePath};
use bevy_prototype_lyon::shapes::Circle;

use crate::variables::binding::VarBinding;

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(drawing_setup);
        app.add_system(update_bound_lines);
        app.add_system(update_bound_circles);
        app.add_system(update_bound_trackers);
    }
}

fn drawing_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component, Clone)]
pub struct BoundPoint {
    x: VarBinding,
    y: VarBinding,
}

impl BoundPoint {
    pub fn new(x: VarBinding, y: VarBinding) -> Self {
        Self { x, y }
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(self.x.value as f32, self.y.value as f32)
    }
}

#[derive(Component, Clone)]
pub struct BoundLine {
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
        let p1: Vec2 = (*line).p1.vec2();
        path_builder.line_to(p1);
        path_builder.line_to(line.p2.vec2());
        *path = path_builder.build();
    }
}

#[derive(Component, Clone)]
pub struct BoundCircle {
    pub radius: VarBinding,
}

impl BoundCircle {
    pub fn new(radius: VarBinding) -> Self {
        Self { radius: radius }
    }
}

fn update_bound_circles(
    mut circle_query: Query<(&mut Path, &mut Transform, &BoundCircle, &BoundPoint)>,
) {
    for (mut path, mut transform, circle, point) in circle_query.iter_mut() {
        let circle = Circle {
            radius: circle.radius.value as f32,
            ..Circle::default()
        };
        // let mut new_path = circle_query.single_mut();
        *path = ShapePath::build_as(&circle);
        let vec: Vec2 = point.vec2();
        *transform = Transform::from_xyz(vec.x, vec.y, 0.);
    }
}

#[derive(Component, Clone)]
pub struct BoundTracker {
    pub target: VarBinding,
    pub history: Vec<f64>,
}
impl BoundTracker {
    pub fn new(target: VarBinding) -> Self {
        Self {
            target,
            history: Vec::new(),
        }
    }
}

fn update_bound_trackers(mut tracker_query: Query<(&mut Path, &mut BoundTracker)>) {
    for (mut line, mut tracker) in &mut tracker_query.iter_mut() {
        let mut path_builder = PathBuilder::new();

        let new_y = tracker.target.value;
        tracker.history.insert(0, new_y);

        for (index, vertex) in tracker.history.iter().enumerate() {
            path_builder.line_to(Vec2::new(index as f32 * 2., *vertex as f32));
        }
        let new_path = path_builder.build();
        *line = new_path;
    }
}
