use bevy::prelude::*;

use self::group::VariableGroup;
use self::variable::Variable;

pub mod binding;
pub mod group;
pub mod list;
pub mod variable;

pub struct VariablePlugin;

impl Plugin for VariablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(variable_devaluate);
    }
}

pub fn variable_devaluate(var_query: Query<&Variable>) {
    for &var in var_query.iter_mut() {
        var.set_recalculated(false);
    }
}

pub fn variable_evaluate(
    var_queries: ParamSet<(
        Query<(Entity, &Variable, &Name)>,
        Query<(Entity, &Variable, &Name)>,
    )>,
) {
}
