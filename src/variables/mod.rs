use bevy::prelude::*;

use self::binding::update_bindings;
pub use self::group::VariableGroup;
pub use self::variable::{Dependent, Independent, Variable};

pub mod binding;
pub mod group;
pub mod lambda;
pub mod list;
pub mod variable;

pub struct VariablePlugin;

impl Plugin for VariablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(devaluate_variables);
        app.add_system(evaluate_variables.after(devaluate_variables));
        app.add_system(update_bindings.after(evaluate_variables));
    }
}

pub fn devaluate_variables(mut var_query: Query<&mut Variable>) {
    for mut var in var_query.iter_mut() {
        var.set_recalculated(false);
    }
}

pub fn evaluate_variables(
    // mut var_queries: ParamSet<(
    //     Query<(Entity, &Variable, &Name), With<Independent>>,
    //     Query<(Entity, &mut Variable, &Name), With<Dependent>>,
    // )>,
    mut var_query: Query<(Entity, &mut Variable)>,
) {
    // let mut vars: Vec<(Entity, Mut<'_, Variable>, &Name)> = var_query.iter_mut().collect();

    // let mut vars: Vec<_> = var_query.iter_mut().collect();
    // let (mut finished, mut unfinished): (_, Vec<_>) =
    //     vars.iter_mut().partition(|x| x.1.recalculated());

    while var_query.iter().any(|w| !w.1.recalculated()) {
        // let mut next_batch: Vec<(Entity, Mut<Variable>)> = Vec::new();
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

    // Old Stuff

    // let unfinished = |i: &Query<(Entity, &mut Variable, &Name)>| {
    //     i.iter().filter(|w| !w.1.recalculated()).count() > 0
    // };
    // while unfinished(&var_query) {
    //     let current_plate = var_query.iter().filter(|w| w.1.recalculated());
    //     let mut next_steps = var_query.iter_mut().filter(|w| {
    //         let needed = (*w.1).equation().children();
    //         needed.iter().all(|v| {
    //             current_plate
    //                 // .iter()
    //                 .map(|u| u.0)
    //                 .collect::<Vec<Entity>>()
    //                 .contains(&v)
    //         })
    //     });
    //     let mut map = HashMap::new();
    //     for (e, var, name) in current_plate {
    //         map.insert(e, var);
    //     }
    //     for (e, mut var, name) in next_steps {
    //         (*var).calculate(&map);
    //     }
    // }
}
