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
use variables::group::Group;
use variables::lambda::{Add, Cos, Mod, Mul, Num, Sin, Var};
use variables::variable::{dependent, independent, Variable};
use variables::VariablePlugin;

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

const GLOBAL: usize = 0;
const UPPER: usize = 1;
const LOWER: usize = 2;

#[derive(Debug)]
struct SineInspector {
    freq1: f64,
    amp1: f64,
    phase1: f64,
    freq2: f64,
    amp2: f64,
    phase2: f64,
}

impl Default for SineInspector {
    fn default() -> Self {
        Self {
            freq1: 2.,
            amp1: 30.,
            phase1: 0.,
            freq2: 2.,
            amp2: 30.,
            phase2: 0.,
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
        // .add_startup_system(setup_gui)
        .add_system(theta_update)
        .add_system(update_sine_inspector)
        .add_system(update_variables_from_gui)
        // .add_system(line_update)
        // .add_system(circle_update)
        .insert_resource(SineInspector::default())
        .add_plugin(EguiPlugin)
        .add_plugin(DrawingPlugin { num_pages: 4 })
        .add_plugin(VariablePlugin)
        .add_plugin(DebugPlugin {
            variables: false,
            bindings: true,
        })
        .run();
}

fn setup_system(mut commands: Commands) {
    let global = Group(GLOBAL);
    let upper = Group(UPPER);
    let lower = Group(LOWER);
    let time = independent(&mut commands, &global, "time", 0.);
    let mut frame_maker = |offset: f64, group: &Group| {
        let phase = independent(&mut commands, group, "phase", 0.);
        let freq = independent(&mut commands, group, "freq", 2.);
        let amp = independent(&mut commands, group, "amp", 30.);
        let circle_x = independent(&mut commands, group, "circle_x", -200.);
        let shift_y = independent(&mut commands, group, "shift_y", offset);
        let point_rad = independent(&mut commands, group, "point_rad", 10.);
        let zero = independent(&mut commands, group, "0", 0.);
        let theta = dependent(
            &mut commands,
            group,
            "theta",
            Mod(Add(Var(phase), Mul(Var(time), Var(freq))), Num(2. * PI)),
        );
        let cos_theta = dependent(
            &mut commands,
            group,
            "cos(theta)",
            Mul(Var(amp), Cos(Var(theta))),
        );
        let sin_theta = dependent(
            &mut commands,
            group,
            "sin(theta)",
            Mul(Var(amp), Sin(Var(theta))),
        );
        let circle_cos = dependent(
            &mut commands,
            group,
            "circle_cos",
            Add(Var(circle_x), Var(cos_theta)),
        );
        let circle_sin = dependent(
            &mut commands,
            group,
            "circle_sin",
            Add(Var(shift_y), Var(sin_theta)),
        );

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
            .insert(BoundTracker::new(circle_sin));
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                Transform::default(),
            ))
            .insert(BoundLine::new(circle_cos, circle_sin, zero, circle_sin));

        return sin_theta;
    };

    let upper_sin = frame_maker(200., &upper);
    let lower_sin = frame_maker(0., &lower);

    let line = PathBuilder::new().build();
    let sum = dependent(
        &mut commands,
        &global,
        "sum",
        Add(Add(Var(upper_sin), Var(lower_sin)), Num(-200.)),
    );
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
            Transform::default(),
        ))
        .insert(BoundTracker::new(sum));
}

fn setup_gui(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            color: Color::rgb(0.3, 0.2, 0.0).into(),
            // color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|w| {
            let circle = Circle::default();

            w.spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
                Transform::default(),
            ));
        });
}

fn theta_update(mut time_query: Query<&mut Variable, With<Time>>) {
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
        .fixed_pos([30.0, 100.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Frequency");
                ui.add(egui::Slider::new(&mut inspector.freq1, 1.0..=30.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp1, 0.5..=100.));
            });
            ui.horizontal(|ui| {
                ui.label("Phase");
                ui.add(egui::Slider::new(&mut inspector.phase1, 0. ..=(PI * 2.)));
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Frequency");
                ui.add(egui::Slider::new(&mut inspector.freq2, 1.0..=30.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp2, 0.5..=100.));
            });
            ui.horizontal(|ui| {
                ui.label("Phase");
                ui.add(egui::Slider::new(&mut inspector.phase2, 0. ..=(PI * 2.)));
            });
        });
}

fn update_variables_from_gui(
    inspector: Res<SineInspector>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
) {
    if inspector.is_changed() {
        // TODO: This is always true?
        vars.p0()
            .iter_mut()
            .filter(|w| w.0 .0 == UPPER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.freq1);
        vars.p1()
            .iter_mut()
            .filter(|w| w.0 .0 == UPPER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.amp1);
        vars.p2()
            .iter_mut()
            .filter(|w| w.0 .0 == UPPER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.phase1);
        vars.p0()
            .iter_mut()
            .filter(|w| w.0 .0 == LOWER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.freq2);
        vars.p1()
            .iter_mut()
            .filter(|w| w.0 .0 == LOWER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.amp2);
        vars.p2()
            .iter_mut()
            .filter(|w| w.0 .0 == LOWER)
            .next()
            .unwrap()
            .1
            .set_value(inspector.phase2);
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
