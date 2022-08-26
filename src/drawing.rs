use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder, ShapePath};
use bevy_prototype_lyon::shapes::Circle;

use crate::variables::{Variable, VariableList};

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_bound_lines);
        app.add_system(update_bound_circles);
        app.add_system(update_bound_trackers);
    }
}

#[derive(Component, Copy, Clone)]
pub struct BoundPoint {
    x: &'static str,
    y: &'static str,
}

impl BoundPoint {
    pub fn new(x: &'static str, y: &'static str) -> Self {
        Self { x, y }
    }

    pub fn vec2(self, vars: &VariableList) -> Vec2 {
        Vec2::new(
            vars.get(self.x.clone()) as f32,
            vars.get(self.y.clone()) as f32,
        )
    }
}

#[derive(Component, Copy, Clone)]
pub struct BoundLine {
    p1: BoundPoint,
    p2: BoundPoint,
}

impl BoundLine {
    pub fn new(p1: BoundPoint, p2: BoundPoint) -> Self {
        Self { p1, p2 }
    }
}

fn update_bound_lines(mut line_query: Query<(&mut BoundLine, &mut Path)>, vars: Res<VariableList>) {
    for (line, mut path) in line_query.iter_mut() {
        let mut path_builder = PathBuilder::new();
        path_builder.line_to(line.p1.vec2(&vars));
        path_builder.line_to(line.p2.vec2(&vars));
        *path = path_builder.build();
    }
}

#[derive(Component, Copy, Clone)]
pub struct BoundCircle {
    pub radius: &'static str,
}

fn update_bound_circles(
    mut circle_query: Query<(&mut Path, &mut Transform, &BoundCircle, &BoundPoint)>,
    vars: Res<VariableList>,
) {
    for (mut path, mut transform, circle, point) in circle_query.iter_mut() {
        let circle = Circle {
            radius: vars.get(circle.radius) as f32,
            ..Circle::default()
        };
        // let mut new_path = circle_query.single_mut();
        *path = ShapePath::build_as(&circle);
        let vec: Vec2 = point.vec2(&vars);
        *transform = Transform::from_xyz(vec.x, vec.y, 0.);
    }
    print!("run.")
}

#[derive(Component, Clone)]
pub struct BoundTracker {
    pub target: &'static str,
    pub history: Vec<f64>,
}
impl BoundTracker {
    pub fn new(target: &'static str) -> Self {
        Self {
            target,
            history: Vec::new(),
        }
    }
}

fn update_bound_trackers(
    mut tracker_query: Query<(&mut Path, &mut BoundTracker)>,
    vars: Res<VariableList>,
) {
    for (mut line, mut tracker) in &mut tracker_query.iter_mut() {
        let mut path_builder = PathBuilder::new();

        let new_y = vars.get(tracker.target);
        tracker.history.insert(0, new_y);

        for (index, vertex) in tracker.history.iter().enumerate() {
            path_builder.line_to(Vec2::new(index as f32 * 2., *vertex as f32));
        }
        let new_path = path_builder.build();
        *line = new_path;
    }
}
