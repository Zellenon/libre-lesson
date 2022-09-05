use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};

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
use crate::{Page, Time, GLOBAL};
const PAGE1: usize = 1;

#[derive(Component)]
struct Freq;
#[derive(Component)]
struct Amp;
#[derive(Component)]
struct Phase;

pub struct Page1Plugin;

impl Plugin for Page1Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_page1_inspector)
            .add_system(update_page1_variables_from_gui)
            .add_startup_system(page1_setup)
            // .add_system_set(SystemSet::on_enter(Page::Simple).with_system(page_enter))
            .insert_resource(Page1Inspector::default());
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

fn page1_setup(mut commands: Commands) {
    let global = Group(GLOBAL);
    let pagegroup = Group(PAGE1);
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
        .insert(Page::Simple)
        .insert(BoundCircle::new(amp))
        .insert(BoundPoint::new(circle_x, zero));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
            Transform::default(),
        ))
        .insert(Page::Simple)
        .insert(BoundCircle::new(point_rad))
        .insert(BoundPoint::new(circle_cos, sin_theta));

    let path_builder = PathBuilder::new();
    let line = path_builder.build();

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Simple)
        .insert(BoundTracker::new(sin_theta, 300));

    commands
        .spawn_bundle(build!(line))
        .insert(Page::Simple)
        .insert(BoundLine::new(circle_cos, sin_theta, zero, sin_theta));
}

#[derive(Debug)]
struct Page1Inspector {
    freq: f64,
    amp: f64,
    phase: f64,
}

impl Default for Page1Inspector {
    fn default() -> Self {
        Self {
            freq: 2.,
            amp: 30.,
            phase: 0.,
        }
    }
}
fn update_page1_inspector(
    mut inspector: ResMut<Page1Inspector>,
    mut egui_context: ResMut<EguiContext>,
    page: Res<State<Page>>,
) {
    let ctx = &mut egui_context.ctx_mut();
    if *page.current() == Page::Simple {
        egui::Window::new("Sine Inspector")
            .fixed_pos([10.0, 100.0])
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
}

fn update_page1_variables_from_gui(
    inspector: Res<Page1Inspector>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
    page: Res<State<Page>>,
) {
    if inspector.is_changed() && *page.current() == Page::Simple {
        // TODO: This is always true?
        vars.p0()
            .iter_mut()
            .filter(|w| w.0 .0 == PAGE1)
            .next()
            .unwrap()
            .1
            .set_value(inspector.freq);
        vars.p1()
            .iter_mut()
            .filter(|w| w.0 .0 == PAGE1)
            .next()
            .unwrap()
            .1
            .set_value(inspector.amp);
        vars.p2()
            .iter_mut()
            .filter(|w| w.0 .0 == PAGE1)
            .next()
            .unwrap()
            .1
            .set_value(inspector.phase);
    }
}
