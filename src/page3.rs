use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use bevy_turborand::RngPlugin;

use crate::drawing::boundcircle::BoundCircle;
use crate::drawing::boundline::BoundLine;
use crate::drawing::boundpoint::BoundPoint;
use crate::drawing::boundtracker::BoundTracker;
use crate::variables::lambda::*;
use crate::variables::{
    group::Group,
    variable::{dependent, independent},
    Variable,
};
use crate::{EquationText, Page, Time, GLOBAL};
const KNOWN: usize = 4;
const UNKNOWN: usize = 5;

#[derive(Component)]
struct Freq;
#[derive(Component)]
struct Amp;
#[derive(Component)]
struct Phase;

pub struct Page3Plugin;

impl Plugin for Page3Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RngPlugin::default())
            .add_system(update_page3_inspector)
            .add_system(update_page3_variables_from_gui)
            .add_system(game_check)
            .add_startup_system(page3_setup)
            .add_startup_system(page3_invisible_setup)
            // .add_startup_system(setup_gui)
            .add_event::<NewGameEvent>()
            .insert_resource(Page3GameState { win: false })
            .insert_resource(Page3Inspector::default());
    }
}

macro_rules! build {
    ($name: expr) => {
        GeometryBuilder::build_as(
            &$name,
            DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
            Transform::default(),
        )
    };
}

fn page3_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let global = Group(GLOBAL);
    let pagegroup = Group(KNOWN);
    let time = independent(&mut commands, &global, "time", 0.);
    let phase = independent(&mut commands, &pagegroup, "phase", 0.);
    let freq = independent(&mut commands, &pagegroup, "freq", 2.);
    let amp = independent(&mut commands, &pagegroup, "amp", 30.);
    let circle_x = independent(&mut commands, &pagegroup, "circle_x", -200.);
    let point_rad = independent(&mut commands, &pagegroup, "point_rad", 10.);
    let zero = independent(&mut commands, &pagegroup, "0", 0.);
    let theta = dependent(
        &mut commands,
        &pagegroup,
        "theta",
        Mod(Add(Var(phase), Mul(Var(time), Var(freq))), Num(2. * PI)),
    );
    let cos_theta = dependent(
        &mut commands,
        &pagegroup,
        "cos(theta)",
        Mul(Var(amp), Cos(Var(theta))),
    );
    let sin_theta = dependent(
        &mut commands,
        &pagegroup,
        "sin(theta)",
        Mul(Var(amp), Sin(Var(theta))),
    );
    let circle_cos = dependent(
        &mut commands,
        &pagegroup,
        "circle_cos",
        Add(Var(circle_x), Var(cos_theta)),
    );

    commands.entity(time).insert(Time);
    commands.entity(amp).insert(Amp);
    commands.entity(freq).insert(Freq);
    commands.entity(phase).insert(Phase);

    let circle = Circle::default();

    commands
        .spawn_bundle(build!(circle))
        .insert(Page::Game)
        .insert(BoundCircle::new(amp))
        .insert(BoundPoint::new(circle_x, zero));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
            Transform::default(),
        ))
        .insert(Page::Game)
        .insert(BoundCircle::new(point_rad))
        .insert(BoundPoint::new(circle_cos, sin_theta));

    let path_builder = PathBuilder::new();
    let line = path_builder.build();

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Game)
        .insert(BoundTracker::new(sin_theta, 300));

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Game)
        .insert(BoundLine::new(circle_cos, sin_theta, zero, sin_theta));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                padding: UiRect {
                    left: Val::Px(25.),
                    ..default()
                },
                ..default()
            },
            color: Color::rgb(0.6, 0.6, 0.6).into(),
            ..Default::default()
        })
        .insert(Page::Game)
        .with_children(|w| {
            w.spawn_bundle(
                TextBundle::from_section(
                    "placeholder text",
                    TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::TOP_LEFT)
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    ..default()
                }),
            )
            .insert(Page::Game)
            .insert(EquationText {
                variables: vec![amp, freq, phase],
                template: "$sin($x + $)",
            });
        });
}

fn page3_invisible_setup(mut commands: Commands) {
    let global = Group(GLOBAL);
    let pagegroup = Group(UNKNOWN);
    let time = independent(&mut commands, &global, "time", 0.);
    let phase = independent(&mut commands, &pagegroup, "phase", 1.2);
    let freq = independent(&mut commands, &pagegroup, "freq", 3.);
    let amp = independent(&mut commands, &pagegroup, "amp", 45.);
    let circle_x = independent(&mut commands, &pagegroup, "circle_x", -200.);
    let shift_y = independent(&mut commands, &pagegroup, "shift_y", -200.);
    let point_rad = independent(&mut commands, &pagegroup, "point_rad", 10.);
    let zero = independent(&mut commands, &pagegroup, "0", 0.);
    let theta = dependent(
        &mut commands,
        &pagegroup,
        "theta",
        Mod(Add(Var(phase), Mul(Var(time), Var(freq))), Num(2. * PI)),
    );
    let cos_theta = dependent(
        &mut commands,
        &pagegroup,
        "cos(theta)",
        Mul(Var(amp), Cos(Var(theta))),
    );
    let sin_theta = dependent(
        &mut commands,
        &pagegroup,
        "sin(theta)",
        Add(Var(shift_y), Mul(Var(amp), Sin(Var(theta)))),
    );
    let circle_cos = dependent(
        &mut commands,
        &pagegroup,
        "circle_cos",
        Add(Var(circle_x), Var(cos_theta)),
    );

    commands.entity(time).insert(Time);
    commands.entity(amp).insert(Amp);
    commands.entity(freq).insert(Freq);
    commands.entity(phase).insert(Phase);

    let circle = Circle::default();

    commands
        .spawn_bundle(build!(circle))
        .insert(Page::Game)
        .insert(BoundCircle::new(amp))
        .insert(BoundPoint::new(circle_x, shift_y));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
            Transform::default(),
        ))
        .insert(Page::Game)
        .insert(BoundCircle::new(point_rad))
        .insert(BoundPoint::new(circle_cos, sin_theta));

    let path_builder = PathBuilder::new();
    let line = path_builder.build();

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Game)
        .insert(BoundTracker::new(sin_theta, 300));

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Game)
        .insert(BoundLine::new(circle_cos, sin_theta, zero, sin_theta));
}

#[derive(Debug)]
struct Page3Inspector {
    freq: f64,
    amp: f64,
    phase: f64,
}

#[derive(Debug)]
struct Page3GameState {
    win: bool,
}

struct NewGameEvent;

impl Default for Page3Inspector {
    fn default() -> Self {
        Self {
            freq: 2.,
            amp: 30.,
            phase: 0.,
        }
    }
}
fn update_page3_inspector(
    mut inspector: ResMut<Page3Inspector>,
    game: Res<Page3GameState>,
    mut egui_context: ResMut<EguiContext>,
    page: Res<State<Page>>,
    mut events: EventWriter<NewGameEvent>,
) {
    let ctx = &mut egui_context.ctx_mut();
    if *page.current() == Page::Game {
        egui::Window::new("Sine Inspector")
            .fixed_pos([10.0, 100.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Frequency");
                    ui.add(egui::Slider::new(&mut inspector.freq, 1.0..=30.).integer());
                });
                ui.horizontal(|ui| {
                    ui.label("Amplitude");
                    ui.add(egui::Slider::new(&mut inspector.amp, 0.5..=100.).integer());
                });
                ui.horizontal(|ui| {
                    ui.label("Phase");
                    ui.add(egui::Slider::new(&mut inspector.phase, 0. ..=(PI * 2.)));
                });
                if game.win {
                    ui.horizontal(|ui| {
                        // ui.add(egui::Button::new("New Game"));
                        if ui.button("New Game").clicked() {
                            events.send(NewGameEvent);
                        }
                    });
                } else {
                }
            });
    }
}

fn new_game(
    mut game_state: ResMut<Page3GameState>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
) {
    println!("New Game");
    // vars.p0()
    //     .iter_mut()
    //     .filter(|w| w.0 .0 == KNOWN)
    //     .next()
    //     .unwrap()
    //     .1
    //     .set_value(inspector.freq);
    // vars.p1()
    //     .iter_mut()
    //     .filter(|w| w.0 .0 == KNOWN)
    //     .next()
    //     .unwrap()
    //     .1
    //     .set_value(inspector.amp);
    // vars.p2()
    //     .iter_mut()
    //     .filter(|w| w.0 .0 == KNOWN)
    //     .next()
    //     .unwrap()
    //     .1
    //     .set_value(inspector.phase);
}

fn update_page3_variables_from_gui(
    inspector: Res<Page3Inspector>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
    page: Res<State<Page>>,
) {
    if inspector.is_changed() && *page.current() == Page::Game {
        // TODO: This is always true?
        vars.p0()
            .iter_mut()
            .filter(|w| w.0 .0 == KNOWN)
            .next()
            .unwrap()
            .1
            .set_value(inspector.freq);
        vars.p1()
            .iter_mut()
            .filter(|w| w.0 .0 == KNOWN)
            .next()
            .unwrap()
            .1
            .set_value(inspector.amp);
        vars.p2()
            .iter_mut()
            .filter(|w| w.0 .0 == KNOWN)
            .next()
            .unwrap()
            .1
            .set_value(inspector.phase);
    }
}

fn game_check(
    mut game_state: ResMut<Page3GameState>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
    page: Res<State<Page>>,
) {
    if *page.current() == Page::Game {
        let get = |vars: &mut ParamSet<(
            Query<(&Group, &mut Variable), With<Freq>>,
            Query<(&Group, &mut Variable), With<Amp>>,
            Query<(&Group, &mut Variable), With<Phase>>,
        )>,
                   group: usize,
                   var: usize| {
            match var {
                0 => vars
                    .p0()
                    .iter()
                    .filter(|w| w.0 .0 == group)
                    .next()
                    .unwrap()
                    .1
                    .value(),
                1 => vars
                    .p1()
                    .iter()
                    .filter(|w| w.0 .0 == group)
                    .next()
                    .unwrap()
                    .1
                    .value(),
                _ => vars
                    .p2()
                    .iter()
                    .filter(|w| w.0 .0 == group)
                    .next()
                    .unwrap()
                    .1
                    .value(),
            }
        };
        let user_freq = get(&mut vars, KNOWN, 0);
        let user_amp = get(&mut vars, KNOWN, 1);
        let user_phase = get(&mut vars, KNOWN, 2);
        let game_freq = get(&mut vars, UNKNOWN, 0);
        let game_amp = get(&mut vars, UNKNOWN, 1);
        let game_phase = get(&mut vars, UNKNOWN, 2);
        game_state.win = (user_freq - game_freq).abs() < 0.1
            && (user_amp - game_amp).abs() < 0.1
            && (user_phase - game_phase).abs() < 0.11
    }
}
