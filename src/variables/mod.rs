use bevy::prelude::*;

pub use self::variable::{Dependent, Independent, Variable};

pub mod binding;
pub mod debug;
pub mod group;
pub mod lambda;
pub mod variable;

pub struct VariablePlugin;

impl Plugin for VariablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("variable_recalculation")
                .with_system(devaluate_variables.label("devaluate"))
                .with_system(evaluate_variables.after("devaluate")),
        );
        app.add_system(devaluate_variables);
        app.add_system(evaluate_variables.after(devaluate_variables));
    }
}

pub fn devaluate_variables(mut var_query: Query<&mut Variable>) {
    for mut var in var_query.iter_mut() {
        var.set_recalculated(false);
    }
}

pub fn evaluate_variables(mut var_query: Query<(Entity, &mut Variable)>) {
    while var_query.iter().any(|w| !w.1.recalculated()) {
        let mut vars: Vec<_> = var_query.iter_mut().collect();
        let (finished, mut unfinished): (_, Vec<_>) =
            vars.iter_mut().partition(|x| x.1.recalculated());
        for var in unfinished.iter_mut().filter(|w| {
            let needed = (*w.1).equation().children();
            needed.iter().all(|v| {
                finished
                    .iter()
                    .map(|u| u.0)
                    .collect::<Vec<Entity>>()
                    .contains(&v)
            })
        }) {
            var.1.calculate(&finished);
        }
    }
}
