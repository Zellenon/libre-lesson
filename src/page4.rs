use std::f64::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_prototype_lyon::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::drawing::boundtracker::BoundTracker;
use crate::variables::lambda::*;
use crate::variables::{
    group::Group,
    variable::{dependent, independent},
    Variable,
};
use crate::{Page, Time, GLOBAL};

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
struct DeleteRowEvent(usize);

pub struct Page4Plugin;

impl Plugin for Page4Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_page4_inspector)
            .add_startup_system(page4_setup)
            .add_system(new_row)
            .add_system(delete_row)
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
        .insert(BoundTracker::new(sum_offset, 300));
}

#[derive(Debug)]
struct Page4Inspector {
    entities: Vec<usize>,
}

impl Default for Page4Inspector {
    fn default() -> Self {
        Self { entities: vec![] }
    }
}
fn update_page4_inspector(
    inspector: ResMut<Page4Inspector>,
    mut egui_context: ResMut<EguiContext>,
    page: Res<State<Page>>,
    mut creation_events: EventWriter<NewRowEvent>,
    mut deletion_events: EventWriter<DeleteRowEvent>,
) {
    let ctx = &mut egui_context.ctx_mut();
    if *page.current() == Page::Fourier {
        egui::Window::new("Sine Inspector")
            .fixed_pos([10.0, 100.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Todo");
                    if ui.button("Press").clicked() {
                        creation_events.send(NewRowEvent);
                    }
                });
                for id in inspector.entities.iter() {
                    if ui.button("Delete").clicked() {
                        deletion_events.send(DeleteRowEvent(*id));
                    }
                }
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
    groups: Query<&Group>,
    mut events: EventReader<NewRowEvent>,
    mut rng: ResMut<GlobalRng>,
    mut inspector: ResMut<Page4Inspector>,
) {
    let mut group_id = 8;
    while groups.iter().any(|w| w.0 == group_id) {
        group_id += 1;
    }
    for _event in events.iter() {
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
        let group = &Group(group_id);
        let phase = independent(&mut commands, group, "phase", rng.i16(1..=200) as f64 / 10.);
        let freq = independent(&mut commands, group, "freq", rng.i16(1..=90) as f64 / 3.);
        let amp = independent(&mut commands, group, "amp", rng.i16(5..=25) as f64);
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
        let sin_theta = dependent(
            &mut commands,
            group,
            "sin(theta)",
            Mul(Var(amp), Sin(Var(theta))),
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

        let path_builder = PathBuilder::new();
        let line = path_builder.build();

        commands
            .spawn_bundle(build!(line))
            .insert(Page::Fourier)
            .insert(group.clone())
            .insert(BoundTracker::new(circle_sin, 300));
        inspector.entities.push(group_id);
    }
}

fn delete_row(
    mut events: EventReader<DeleteRowEvent>,
    mut inspector: ResMut<Page4Inspector>,
    deletion_candidates: Query<(Entity, &Group)>,
    mut commands: Commands,
) {
    for DeleteRowEvent(id) in events.iter() {
        for (e, _g) in deletion_candidates.iter().filter(|(_e, g)| g.0 == *id) {
            commands.entity(e).despawn();
        }
        inspector.entities.retain(|&w| w != *id)
    }
}

fn update_sum(
    mut queries: ParamSet<(
        Query<&mut Variable, With<SumSin>>,
        Query<&Variable, With<SinOutput>>,
    )>,
) {
    let sum: f64 = queries.p1().iter().map(|w| w.value()).sum();
    queries.p0().single_mut().set_value(sum);
}
