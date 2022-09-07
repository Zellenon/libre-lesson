use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use bevy_turborand::{DelegatedRng, GlobalRng};

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

const SUM: usize = 7;
const PROC: usize = 8;

#[derive(Component)]
struct Freq;
#[derive(Component)]
struct Amp;
#[derive(Component)]
struct Phase;

#[derive(Component)]
struct SumSin;
#[derive(Component)]
struct SinOutput;
#[derive(Component)]
struct Offset;
struct NewRowEvent;
struct DeleteRowEvent(Entity);

pub struct Page4Plugin;

impl Plugin for Page4Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_page4_inspector)
            .add_startup_system(page4_setup)
            .add_system(new_row)
            .add_system(update_sum.after("variable_recalculation").before("drawing"))
            .add_event::<NewRowEvent>()
            .add_event::<DeleteRowEvent>()
            .insert_resource(Page4Inspector::default());
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

fn page4_setup(mut commands: Commands) {
    let global = Group(GLOBAL);
    let sum_group = Group(SUM);
    let procedural_group = Group(PROC);
    // let time = independent(&mut commands, &global, "time", 0.);

    let line = PathBuilder::new().build();
    let sum = independent(&mut commands, &global, "sum", 0.);
    commands.entity(sum).insert(SumSin);
    let sum_offset = dependent(
        &mut commands,
        &global,
        "sum_offset",
        Add(Num(300.), Var(sum)),
    );
    commands
        .spawn_bundle(build!(line))
        .insert(Page::Fourier)
        .insert(BoundTracker::new(sum_offset, 250));
}

#[derive(Debug)]
struct Page4Inspector {
    entities: Vec<Entity>,
}

impl Default for Page4Inspector {
    fn default() -> Self {
        Self { entities: vec![] }
    }
}
fn update_page4_inspector(
    mut inspector: ResMut<Page4Inspector>,
    mut egui_context: ResMut<EguiContext>,
    page: Res<State<Page>>,
    mut events: EventWriter<NewRowEvent>,
) {
    let ctx = &mut egui_context.ctx_mut();
    if *page.current() == Page::Fourier {
        egui::Window::new("Sine Inspector")
            .fixed_pos([10.0, 100.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Todo");
                    if ui.button("Press").clicked() {
                        events.send(NewRowEvent);
                    }
                });
            });
    }
}

fn new_row(
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<&Variable, With<SumSin>>,
        Query<&Variable, With<Offset>>,
        Query<Entity, With<Time>>,
    )>,
    mut events: EventReader<NewRowEvent>,
    mut rng: ResMut<GlobalRng>,
) {
    for event in events.iter() {
        let mut offset = 200.;
        while queries
            .p1()
            .iter()
            .filter(|w| w.value() - offset < 1.)
            .count()
            > 0
        {
            offset -= 75.;
        }
        let group = &Group(PROC);
        let phase = independent(&mut commands, group, "phase", 0.);
        let freq = independent(&mut commands, group, "freq", rng.i16(1..=30) as f64 / 2.);
        let amp = independent(&mut commands, group, "amp", rng.i16(5..=25) as f64);
        let circle_x = independent(&mut commands, group, "circle_x", -200.);
        let shift_y = independent(&mut commands, group, "shift_y", offset);
        let theta = dependent(
            &mut commands,
            group,
            "theta",
            Mod(
                Add(
                    Var(phase),
                    Mul(Var(queries.p2().iter().next().unwrap()), Var(freq)),
                ),
                Num(2. * PI),
            ),
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

        // commands.entity(time).insert(Time);
        commands.entity(amp).insert(Amp);
        commands.entity(freq).insert(Freq);
        commands.entity(phase).insert(Phase);
        commands.entity(sin_theta).insert(SinOutput);
        commands.entity(shift_y).insert(Offset);

        // let circle = Circle::default();

        // commands
        //     .spawn_bundle(build!(circle))
        //     .insert(Page::Fourier)
        //     .insert(BoundCircle::new(amp))
        //     .insert(BoundPoint::new(circle_x, shift_y));

        // commands
        //     .spawn_bundle(GeometryBuilder::build_as(
        //         &circle,
        //         DrawMode::Stroke(StrokeMode::new(Color::RED, 3.)),
        //         Transform::default(),
        //     ))
        //     .insert(Page::Fourier)
        //     .insert(BoundCircle::new(point_rad))
        //     .insert(BoundPoint::new(circle_cos, circle_sin));

        let path_builder = PathBuilder::new();
        let line = path_builder.build();

        commands
            .spawn_bundle(build!(line))
            .insert(Page::Fourier)
            .insert(BoundTracker::new(circle_sin, 300));

        // commands
        //     .spawn_bundle(build!(line))
        //     .insert(Page::Fourier)
        //     .insert(BoundLine::new(circle_cos, circle_sin, zero, circle_sin));
    }
}

fn delete_row() {}

fn update_sum(
    mut queries: ParamSet<(
        Query<&mut Variable, With<SumSin>>,
        Query<&Variable, With<SinOutput>>,
    )>,
) {
    let sum: f64 = queries.p1().iter().map(|w| w.value()).sum();
    queries.p0().single_mut().set_value(sum);
}
