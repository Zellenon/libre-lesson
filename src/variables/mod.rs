use bevy::ecs::query::QueryIter;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

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
    let vars: Vec<_> = var_query.iter_mut().collect();
    let (mut finished, mut unfinished): (_, Vec<_>) =
        vars.into_iter().partition(|x| x.1.recalculated());

    while unfinished.len() > 0 {
        let next_batch = Vec::new();
        let context = finished.iter().collect::<HashMap<_, _>>();
        for var in unfinished.iter() {}
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
