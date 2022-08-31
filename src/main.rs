use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, prelude::Component};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use drawing::DrawingPlugin;
use std::marker::PhantomData;
use std::sync::Arc;
use variables::list::VariableList;

use crate::drawing::{BoundCircle, BoundLine, BoundPoint, BoundTracker};
use crate::variables::group::VariableGroup;

mod drawing;
mod variables;

#[derive(Component)]
struct SineLine(Vec<f64>);

#[derive(Debug)]
struct SineInspector {
    freq: f64,
    amp: f64,
}

impl Default for SineInspector {
    fn default() -> Self {
        Self { freq: 2., amp: 30. }
    }
}

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_system(theta_update)
        .add_system(line_update)
        // .add_system(circle_update)
        .insert_resource(SineInspector::default())
        .add_plugin(EguiPlugin)
        .add_system(update_sine_inspector)
        .add_system(update_variables_from_gui)
        .add_plugin(DrawingPlugin)
        .run();
}

fn setup_system(mut commands: Commands) {
    let mut frame_maker = |prefix: String, offset: f64| -> VariableGroup {
        let mut vars: VariableGroup = VariableGroup::new();
        let independent_vars = [
            ("theta", 0.),
            ("freq", 2.),
            ("amp", 30.),
            ("circle_x", -100.),
            ("shift_y", offset),
            ("point,rad", 10.),
        ];
        let dependent_vars: [(&str, Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>); 4] = [
            (
                "cos(theta)",
                Arc::new(move |vars: &VariableList| vars.get("theta").cos() * vars.get("amp")),
            ),
            (
                "sin(theta)",
                Arc::new(move |vars: &VariableList| vars.get("theta").sin() * vars.get("amp")),
            ),
            (
                "circle_cos",
                Arc::new(move |vars: &VariableList| vars.get("circle_x") + vars.get("cos(theta)")),
            ),
            (
                "circle_sin",
                Arc::new(move |vars: &VariableList| vars.get("sin(theta)")),
            ),
        ];

        let circle = Circle::default();

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
                Transform::default(),
            ))
            .insert(BoundCircle::new("amp"))
            .insert(BoundPoint::new("circle_x", "shift_y"));

        let point = BoundPoint::new("circle_cos", "circle_sin");
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
                Transform::default(),
            ))
            .insert(BoundCircle::new("point_rad"))
            .insert(point.clone());

        let path_builder = PathBuilder::new();
        let line = path_builder.build();

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.0)),
                Transform::default(),
            ))
            .insert(BoundTracker::new("sin(theta)"));
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                Transform::default(),
            ))
            .insert(BoundLine::new(point, BoundPoint::new("0", "sin(theta)")));
        return vars;
    };

    let vars_upper = frame_maker(200.);
    let vars_lower = frame_maker(-200.);
    let mut master_vars = VariableGroup::new();
    master_vars.add_child("upper", vars_upper);
    master_vars.add_child("lower", vars_lower);
    commands.insert_resource(master_vars);
    // commands.insert_resource(vars_upper);
}

fn theta_update(
    // time: Res<Time>,
    mut vars: ResMut<VariableGroup>,
) {
    // let delta = time.delta_seconds_f64();
    let delta = 0.02;
    let theta = vars.get("theta");
    let freq = vars.get("freq");
    vars.insert(
        "theta",
        Variable::Independent(theta + delta * 3.1415 * freq),
    );
}

fn line_update(mut sinequery: Query<(&mut Path, &mut SineLine)>, vars: Res<VariableGroup>) {
    let theta = vars.get("theta");
    let amp = vars.get("amp");

    for (mut line, mut sine) in sinequery.iter_mut() {
        let mut path_builder = PathBuilder::new();
        let new_y = (theta).sin() * amp;
        sine.0.insert(0, new_y);

        for (index, vertex) in sine.0.iter().enumerate() {
            path_builder.line_to(Vec2::new(index as f32 * 2., *vertex as f32));
        }
        let new_path = path_builder.build();
        *line = new_path;
    }
}

fn update_sine_inspector(
    mut inspector: ResMut<SineInspector>,
    mut egui_context: ResMut<EguiContext>,
) {
    let ctx = &mut egui_context.ctx_mut();

    egui::Window::new("Sine Inspector")
        .fixed_pos([50.0, 200.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Frequency");
                ui.add(egui::Slider::new(&mut inspector.freq, 1.0..=15.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp, 0.5..=100.));
            });
        });
}

fn update_variables_from_gui(inspector: Res<SineInspector>, mut vars: ResMut<VariableGroup>) {
    if inspector.is_changed() {
        vars.insert("freq", Variable::Independent(inspector.freq));
        vars.insert("amp", Variable::Independent(inspector.amp));
    }
}

struct VariableUpdateEvent<T: Component>(f64, PhantomData<T>);

fn update_variables<T: Component>(
    var_query: Query<(&T, &Variable)>,
    events: EventReader<VariableUpdateEvent<T>>,
) {
    for (marker, var) in var_query.iter_mut() {
        if let Variable::Independent { value: v } = var {
            let new_value = events.iter().next().unwrap();
            var.value = new_value;
        }
    }
}
