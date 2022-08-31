use bevy::prelude::*;

use self::list::VariableList;
use self::variable::Variable;

mod binding;
mod list;
mod variable;

pub struct VariablePlugin;

impl Plugin for VariablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(variable_devaluate);
    }
}

pub fn variable_devaluate(var_query: Query<&VariableList>) {
    for &vars in var_query.iter_mut() {
        for (name, var) in vars.variables {
            if let Variable::Dependent {
                value: x1,
                recalculated: r,
                equation: x2,
                parent,
            } = var
            {
                if r {
                    var = Variable::Dependent {
                        value: x1,
                        recalculated: false,
                        equation: x2,
                        parent,
                    }
                }
            }
        }
    }
}
