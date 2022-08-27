use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use drawing::DrawingPlugin;
use std::sync::Arc;
use variables::{Variable, VariableList};

use crate::drawing::{BoundCircle, BoundLine, BoundPoint, BoundTracker};

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
    use variables::Variable::*;

    let mut frameMaker = |prefix: &str, offset: f64| -> VariableList {
        let mut vars: VariableList = VariableList::new();
        vars.insert("theta", Independent(0.));
        vars.insert("freq", Independent(2.));
        vars.insert("amp", Independent(30.));
        vars.insert("circle_x", Independent(-100.));
        vars.insert("point_rad", Independent(10.));
        vars.insert(
            "cos(theta)",
            Variable::Dependent(Arc::new(move |vars: &VariableList| {
                vars.get("theta").cos() * vars.get("amp")
            })),
        );
        vars.insert(
            "sin(theta)",
            Variable::Dependent(Arc::new(move |vars: &VariableList| {
                vars.get("theta").sin() * vars.get("amp")
            })),
        );
        vars.insert(
            "circle_cos",
            Variable::Dependent(Arc::new(move |vars: &VariableList| {
                vars.get("circle_x") + vars.get("cos(theta)")
            })),
        );
        vars.insert(
            "circle_sin",
            Variable::Dependent(Arc::new(move |vars: &VariableList| vars.get("sin(theta)"))),
        );

        let circle = Circle {
            radius: 30.,
            ..Circle::default()
        };

        let transform = Transform::from_xyz(0., offset as f32, 0.);

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
                transform,
            ))
            .insert(BoundCircle::new("amp"))
            .insert(BoundPoint::new("circle_x", "0"));

        let point = BoundPoint::new("circle_cos", "circle_sin");
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
                transform,
            ))
            .insert(BoundCircle::new("point_rad"))
            .insert(point.clone());

        let path_builder = PathBuilder::new();
        let line = path_builder.build();

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.0)),
                transform,
            ))
            .insert(BoundTracker::new("sin(theta)"));
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                transform,
            ))
            .insert(BoundLine::new(point, BoundPoint::new("0", "sin(theta)")));
        return vars;
    };

    let vars_upper = frameMaker("upper_", 200.);
    commands.insert_resource(vars_upper);
}

fn theta_update(
    // time: Res<Time>,
    mut vars: ResMut<VariableList>,
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

fn line_update(mut sinequery: Query<(&mut Path, &mut SineLine)>, vars: Res<VariableList>) {
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
                ui.add(egui::Slider::new(&mut inspector.freq, 0.5..=15.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp, 0.5..=100.));
            });
        });
}

fn update_variables_from_gui(inspector: Res<SineInspector>, mut vars: ResMut<VariableList>) {
    if inspector.is_changed() {
        vars.insert("freq", Variable::Independent(inspector.freq));
        vars.insert("amp", Variable::Independent(inspector.amp));
    }
}
