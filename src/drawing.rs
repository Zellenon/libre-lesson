use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Path, PathBuilder, ShapePath};
use bevy_prototype_lyon::shapes::Circle;
use std::marker;

use crate::variables::{MathVar, Variable, VariableList};

pub struct DrawingPlugin<T: MathVar> {
    pub _marker: marker::PhantomData<T>,
}

impl<T: MathVar + 'static> Plugin for DrawingPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system(update_bound_lines::<T>);
    }
}

#[derive(Component, Copy, Clone)]
struct BoundPoint<T> {
    x: T,
    y: T,
}

impl<T: MathVar> BoundPoint<T> {
    pub fn vec2(self, vars: VariableList<T>) -> Vec2 {
        Vec2::new(
            vars.get(self.x.clone()) as f32,
            vars.get(self.y.clone()) as f32,
        )
    }
}

#[derive(Component, Copy, Clone)]
struct BoundLine<T> {
    p1: BoundPoint<T>,
    p2: BoundPoint<T>,
}

impl<T: MathVar> BoundLine<T> {
    pub fn new(p1: BoundPoint<T>, p2: BoundPoint<T>) -> Self {
        Self { p1, p2 }
    }
}

fn update_bound_lines<T: MathVar + 'static>(
    mut line_query: Query<(&mut BoundLine<T>, &mut Path)>,
    _vars: Res<VariableList<T>>,
) {
    for (line, mut path) in line_query.iter_mut() {
        let mut path_builder = PathBuilder::new();
        let vars = _vars.clone();
        let p1: Vec2 = (*line).p1.vec2(vars);
        path_builder.line_to(line.p1.vec2(vars));
        path_builder.line_to(line.p2.vec2(vars));
        *path = path_builder.build();
    }
}

#[derive(Component, Copy, Clone)]
pub struct BoundCircle {
    radius: f64,
}

fn update_bound_circles<T: MathVar + 'static>(
    mut circle_query: Query<(&mut Path, &mut Transform, &BoundCircle, &BoundPoint<T>)>,
    vars: Res<VariableList<T>>,
) {
    for (mut path, mut transform, circle, point) in circle_query.iter_mut() {
        let circle = Circle {
            radius: circle.radius as f32,
            ..Circle::default()
        };
        // let mut new_path = circle_query.single_mut();
        *path = ShapePath::build_as(&circle);
        let vec: Vec2 = point.vec2(*vars);
        *transform = Transform::from_xyz(vec.x, vec.y, 0.);
    }
}
