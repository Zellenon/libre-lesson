use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, prelude::Component};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use drawing::boundcircle::BoundCircle;
use drawing::boundline::BoundLine;
use drawing::boundpoint::BoundPoint;
use drawing::boundtracker::BoundTracker;
use drawing::DrawingPlugin;
use std::f64::consts::PI;
use std::marker::PhantomData;
use variables::debug::DebugPlugin;
use variables::lambda::{Add, Cos, Mod, Mul, Num, Sin, Var};
use variables::variable::{dependent, independent, Variable};
use variables::VariablePlugin;

use crate::variables::group::VariableGroup;

mod drawing;
mod variables;

#[derive(Component)]
struct SineLine(Vec<f64>);

#[derive(Component)]
struct Time;

#[derive(Component)]
struct Freq;

#[derive(Component)]
struct Amp;
#[derive(Component)]
struct Phase;

#[derive(Debug)]
struct SineInspector {
    freq: f64,
    amp: f64,
    phase: f64,
}

impl Default for SineInspector {
    fn default() -> Self {
        Self {
            freq: 2.,
            amp: 30.,
            phase: 0.,
        }
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
        .add_system(update_sine_inspector)
        .add_system(update_variables_from_gui)
        // .add_system(line_update)
        // .add_system(circle_update)
        .insert_resource(SineInspector::default())
        .add_plugin(EguiPlugin)
        .add_plugin(DrawingPlugin)
        .add_plugin(VariablePlugin)
        .add_plugin(DebugPlugin {
            variables: false,
            bindings: true,
        })
        .run();
}

fn setup_system(mut commands: Commands) {
    let mut frame_maker = |offset: f64| {
        let time = independent(&mut commands, "time", 0.);
        let phase = independent(&mut commands, "phase", 0.);
        let freq = independent(&mut commands, "freq", 2.);
        let amp = independent(&mut commands, "amp", 30.);
        let circle_x = independent(&mut commands, "circle_x", -200.);
        let shift_y = independent(&mut commands, "shift_y", offset);
        let point_rad = independent(&mut commands, "point_rad", 10.);
        let zero = independent(&mut commands, "0", 0.);
        let theta = dependent(
            &mut commands,
            "theta",
            Mod(Add(Var(phase), Mul(Var(time), Var(freq))), Num(2. * PI)),
        );
        let cos_theta = dependent(&mut commands, "cos(theta)", Mul(Var(amp), Cos(Var(theta))));
        let sin_theta = dependent(
            &mut commands,
            "sin(theta)",
            Add(Var(shift_y), Mul(Var(amp), Sin(Var(theta)))),
        );
        let circle_cos = dependent(
            &mut commands,
            "circle_cos",
            Add(Var(circle_x), Var(cos_theta)),
        );
        let circle_sin = dependent(&mut commands, "circle_sin", Var(sin_theta));

        commands.entity(time).insert(Time);
        commands.entity(amp).insert(Amp);
        commands.entity(freq).insert(Freq);
        commands.entity(phase).insert(Phase);

        let circle = Circle::default();

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
                Transform::default(),
            ))
            .insert(BoundCircle::new(amp))
            .insert(BoundPoint::new(circle_x, shift_y));

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
                Transform::default(),
            ))
            .insert(BoundCircle::new(point_rad))
            .insert(BoundPoint::new(circle_cos, circle_sin));

        let path_builder = PathBuilder::new();
        let line = path_builder.build();

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.0)),
                Transform::default(),
            ))
            .insert(BoundTracker::new(sin_theta));
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                Transform::default(),
            ))
            .insert(BoundLine::new(circle_cos, circle_sin, zero, sin_theta));
    };

    // frame_maker(0.);
    frame_maker(200.);
}

fn theta_update(mut time_query: Query<&mut Variable, With<Time>>) {
    // let delta = time.delta_seconds_f64();
    let delta = 0.02;
    for mut var in time_query.iter_mut() {
        let old_value = (*var).value();
        var.set_value(old_value + delta);
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
                ui.add(egui::Slider::new(&mut inspector.freq, 1.0..=30.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp, 0.5..=100.));
            });
            ui.horizontal(|ui| {
                ui.label("Phase");
                ui.add(egui::Slider::new(&mut inspector.phase, 0. ..=(PI * 2.)));
            });
        });
}

fn update_variables_from_gui(
    inspector: Res<SineInspector>,
    mut vars: ParamSet<(
        Query<&mut Variable, With<Freq>>,
        Query<&mut Variable, With<Amp>>,
        Query<&mut Variable, With<Phase>>,
    )>,
) {
    if inspector.is_changed() {
        // TODO: This is always true?
        vars.p0().single_mut().set_value(inspector.freq);
        vars.p1().single_mut().set_value(inspector.amp);
        vars.p2().single_mut().set_value(inspector.phase);
    }
}

struct VariableUpdateEvent<T: Component>(f64, PhantomData<T>);

fn update_variables<T: Component>(
    mut var_query: Query<(&T, &mut Variable)>,
    mut events: EventReader<VariableUpdateEvent<T>>,
) {
    let val = events.iter().last().unwrap().0;
    for (marker, mut var) in var_query.iter_mut() {
        (var).set_value(val);
    }
}
