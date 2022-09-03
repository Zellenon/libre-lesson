use bevy::prelude::*;
use std::sync::Arc;

use super::variable::Variable;

#[derive(Component)]
struct VarBinding {
    variable: Entity,
    value: f64,
}

fn update_bindings(
    mut binding_query: Query<&mut VarBinding>,
    var_query: Query<(Entity, &Variable)>,
) {
    for (e, var) in var_query.iter() {
        for binding in binding_query.iter_mut().filter(|w| w.variable == e) {
            match var {
                Variable::Dependent {
                    value: new_val,
                    recalculated: _,
                    equation: _,
                } => (),
                Variable::Independent { value: new_val } => (),
            };
        }
    }
}
