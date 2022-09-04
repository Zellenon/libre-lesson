use bevy::prelude::*;

use super::variable::Variable;

#[derive(Component, Clone, PartialEq)]
pub struct VarBinding {
    pub variable: Entity,
    pub value: f64,
}

impl VarBinding {
    pub fn new(master: Entity) -> Self {
        Self {
            variable: master,
            value: 0.,
        }
    }
}

pub fn update_bindings(
    mut binding_query: Query<&mut VarBinding>,
    var_query: Query<(Entity, &Variable)>,
) {
    for (e, var) in var_query.iter() {
        for binding in binding_query.iter_mut().filter(|w| w.variable == e) {
            match var {
                Variable::Dependent {
                    value: new_value,
                    recalculated: _,
                    equation: _,
                } => binding.value = *new_value,
                Variable::Independent { value: new_value } => binding.value = *new_value,
            };
        }
    }
}

pub fn bind(commands: &mut Commands, master: Entity) -> VarBinding {
    let binding = VarBinding::new(master);
    commands.spawn().insert(binding);
    binding
}
