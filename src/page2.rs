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

const UPPER: usize = 2;
const LOWER: usize = 3;

#[derive(Component)]
struct Freq;
#[derive(Component)]
struct Amp;
#[derive(Component)]
struct Phase;

pub struct Page2Plugin;

impl Plugin for Page2Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_page2_inspector)
            .add_system(update_page2_variables_from_gui)
            .add_startup_system(page2_setup)
            .insert_resource(Page2Inspector::default());
    }
}

fn page2_setup(mut commands: Commands) {
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
            .insert(Page::Combination)
            .insert(BoundCircle::new(amp))
            .insert(BoundPoint::new(circle_x, shift_y));

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &circle,
                DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
                Transform::default(),
            ))
            .insert(Page::Combination)
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
            .insert(Page::Combination)
            .insert(BoundTracker::new(circle_sin, 300));

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::WHITE, 2.0)),
                Transform::default(),
            ))
            .insert(Page::Combination)
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
        .insert(Page::Combination)
        .insert(BoundTracker::new(sum, 300));
}

#[derive(Debug)]
struct Page2Inspector {
    freq1: f64,
    amp1: f64,
    phase1: f64,
    freq2: f64,
    amp2: f64,
    phase2: f64,
}

impl Default for Page2Inspector {
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
fn update_page2_inspector(
    mut inspector: ResMut<Page2Inspector>,
    mut egui_context: ResMut<EguiContext>,
    page: Res<State<Page>>,
) {
    let ctx = &mut egui_context.ctx_mut();
    if *page.current() == Page::Combination {
        egui::Window::new("Sine Inspector")
            .fixed_pos([10.0, 100.0])
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
}

fn update_page2_variables_from_gui(
    inspector: Res<Page2Inspector>,
    mut vars: ParamSet<(
        Query<(&Group, &mut Variable), With<Freq>>,
        Query<(&Group, &mut Variable), With<Amp>>,
        Query<(&Group, &mut Variable), With<Phase>>,
    )>,
    page: Res<State<Page>>,
) {
    if inspector.is_changed() && *page.current() == Page::Combination {
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
