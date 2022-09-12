//! This crate is a way to have generic and dynamic calculations based on various factors
//! and bind game entities to the outcome of said calculations.
use bevy::prelude::*;

pub use self::variable::{Dependent, Independent, Variable};

/// Traits and methods to use Variable and Equation values with other components.
pub mod binding;
/// Plugins for debugging calculations and systems.
pub mod debug;
/// Used to mark subspaces of data.
pub mod group;
/// The package handling data-oriented declaration of dynamic equations.
pub mod lambda;
/// The core of calculations. Holds equations and values.
pub mod variable;

/// Adds variable recalculation systems.
pub struct VariablePlugin;

impl Plugin for VariablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .label("variable_recalculation")
                .with_system(devaluate_variables.label("devaluate"))
                .with_system(evaluate_variables.after("devaluate")),
        );
    }
}

/// Marks all variables as "not evaluated yet for the current cycle".
pub fn devaluate_variables(mut var_query: Query<&mut Variable>) {
    for mut var in var_query.iter_mut() {
        var.set_recalculated(false);
    }
}

/// Keep evaluating variables until they all have an f64 value for the current cycle.
pub fn evaluate_variables(mut var_query: Query<(Entity, &mut Variable)>) {
    while var_query.iter().any(|w| !w.1.recalculated()) {
        let mut vars: Vec<_> = var_query.iter_mut().collect();
        let (finished, mut unfinished): (_, Vec<_>) =
            vars.iter_mut().partition(|x| x.1.recalculated());
        if unfinished
            .iter_mut()
            .filter(|w| {
                let needed = (*w.1).equation().children();
                needed.iter().all(|v| {
                    finished
                        .iter()
                        .map(|u| u.0)
                        .collect::<Vec<Entity>>()
                        .contains(&v)
                })
            })
            .count()
            == 0
        {
            return;
        }
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
