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

pub fn devaluate_variables(var_query: Query<&Variable>) {
    for &var in var_query.iter_mut() {
        var.set_recalculated(false);
    }
}

pub fn evaluate_variables(
    var_queries: ParamSet<(
        Query<(Entity, &Variable, &Name), With<Independent>>,
        Query<(Entity, &Variable, &Name), With<Dependent>>,
    )>,
) {
    let unfinished =
        |i: QueryIter<(Entity, &Variable, &Name), _>| i.filter(|w| !w.1.recalculated()).count() > 0;
    let plate = || {
        let mut base: Vec<(Entity, &Variable, &Name)> = var_queries.p0().iter().collect();
        let mut base2: Vec<(Entity, &Variable, &Name)> = var_queries
            .p1()
            .iter()
            .filter(|w| !w.1.recalculated())
            .collect();
        base.append(&mut base2);
        base
    };
    while unfinished(var_queries.p1().iter()) {
        let current_plate = plate();
        let next_steps = var_queries.p1().iter_mut().filter(|w| {
            let needed = (*w.1).equation().children();
            needed.iter().all(|v| {
                current_plate
                    .iter()
                    .map(|u| u.0)
                    .collect::<Vec<Entity>>()
                    .contains(&v)
            })
        });
        let mut map = HashMap::new();
        for (e, var, name) in current_plate.iter() {
            map.insert(*e, *var);
        }
        for (e, var, name) in next_steps {
            (*var).calculate(map);
        }
    }
}
