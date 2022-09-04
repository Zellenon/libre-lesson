use std::time::Duration;

use bevy::prelude::*;

use super::Variable;

pub struct DebugPlugin {
    pub variables: bool,
    pub bindings: bool,
}

#[derive(Component)]
struct DebugTimer(Timer);

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(debug_setup);
        if self.variables {
            app.add_system(variable_print);
        }
    }
}

fn debug_setup(mut commands: Commands) {
    commands.insert_resource(DebugTimer(Timer::new(Duration::from_secs(1), true)))
}

fn variable_print(
    var_query: Query<(&Variable, &Name)>,
    mut timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        for (var, name) in var_query.iter() {
            println!("{}: {}", name, var.value());
        }
    }
}
