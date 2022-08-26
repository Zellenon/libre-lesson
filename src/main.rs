use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use drawing::DrawingPlugin;
// use drawing::*;
use std::convert::From;
use std::marker::PhantomData;
use std::{hash::Hash, sync::Arc};
use strum_macros::EnumIter;
use variables::{MathVar, Variable, VariableList};

mod drawing;
mod variables;

#[derive(Component)]
struct SineLine(Vec<f64>);

#[derive(EnumIter, Debug, PartialEq, Eq, Hash, Clone)]
enum Vars {
    Theta,
    Amp,
    Freq,
    CircleX,
    Sin,
    Cos,
    CircleSin,
    CircleCos,
}

impl MathVar for Vars {}

#[derive(Component)]
struct Theta;

#[derive(Component)]
struct Amp;

#[derive(Component)]
struct Freq;

#[derive(Component)]
struct BaseCircle;

#[derive(Component)]
struct CircleX;
#[derive(Component)]
struct CircleSin;
#[derive(Component)]
struct CircleCos;

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
        .add_system(circle_update)
        .insert_resource(SineInspector::default())
        .add_plugin(EguiPlugin)
        .add_system(update_sine_inspector)
        .add_system(update_variables_from_gui)
        .add_plugin(DrawingPlugin {
            _marker: PhantomData as PhantomData<Vars>,
        })
        .run();
}

fn setup_system(mut commands: Commands) {
    use variables::Variable::*;
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::new(-400., -400.));
    path_builder.line_to(100.0 * Vec2::ZERO);
    let line = path_builder.build();

    let mut vars: VariableList<Vars> = VariableList::new();
    vars.insert(Vars::Theta, Independent(0.));
    vars.insert(Vars::Freq, Independent(2.));
    vars.insert(Vars::Amp, Independent(30.));
    vars.insert(Vars::CircleX, Independent(-300.));
    vars.insert(
        Vars::Cos,
        Variable::Dependent(Arc::new(move |vars: &VariableList<Vars>| {
            vars.get(Vars::Theta).cos()
        })),
    );
    vars.insert(
        Vars::Sin,
        Variable::Dependent(Arc::new(move |vars: &VariableList<Vars>| {
            vars.get(Vars::Theta).sin()
        })),
    );
    vars.insert(
        Vars::CircleCos,
        Variable::Dependent(Arc::new(move |vars: &VariableList<Vars>| {
            vars.get(Vars::CircleX) + vars.get(Vars::Cos)
        })),
    );
    vars.insert(
        Vars::CircleSin,
        Variable::Dependent(Arc::new(move |vars: &VariableList<Vars>| {
            vars.get(Vars::CircleX) + vars.get(Vars::Sin)
        })),
    );

    commands.insert_resource(vars);

    let circle = Circle {
        radius: 30.,
        ..Circle::default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.)),
            Transform::default(),
        ))
        .insert(BaseCircle);

    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &line,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 3.0)),
            Transform::default(),
        ))
        .insert(SineLine(Vec::from([0.])));
}

fn theta_update(
    // time: Res<Time>,
    mut vars: ResMut<VariableList<Vars>>,
) {
    // let delta = time.delta_seconds_f64();
    let delta = 0.02;
    let theta = vars.get_raw(Vars::Theta);
    match theta {
        Variable::Independent(t) => {
            let new_theta = t + delta * 3.1415 * vars.get(Vars::Freq);
            vars.insert(Vars::Theta, Variable::Independent(new_theta));
        }
        Variable::Dependent(_) => {}
    };
}

fn line_update(mut sinequery: Query<(&mut Path, &mut SineLine)>, vars: Res<VariableList<Vars>>) {
    let theta = vars.get(Vars::Theta);
    let amp = vars.get(Vars::Amp);

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

fn circle_update(
    mut circle_query: Query<&mut Path, With<BaseCircle>>,
    vars: Res<VariableList<Vars>>,
) {
    let amp = vars.get(Vars::Amp);
    let circle = Circle {
        radius: amp as f32,
        ..Circle::default()
    };
    let mut path = circle_query.single_mut();
    *path = ShapePath::build_as(&circle);
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
                ui.add(egui::Slider::new(&mut inspector.freq, 0.5..=20.));
            });
            ui.horizontal(|ui| {
                ui.label("Amplitude");
                ui.add(egui::Slider::new(&mut inspector.amp, 0.5..=50.));
            });
        });
}

fn update_variables_from_gui(inspector: Res<SineInspector>, mut vars: ResMut<VariableList<Vars>>) {
    if inspector.is_changed() {
        vars.insert(Vars::Freq, Variable::Independent(inspector.freq));
        vars.insert(Vars::Amp, Variable::Independent(inspector.amp));
    }
}
