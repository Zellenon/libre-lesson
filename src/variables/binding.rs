use bevy::prelude::*;

use super::variable::Variable;

pub trait Bound {
    fn get_bindings(&self) -> Vec<Entity>;
    fn set_bindings(&mut self, bindings: Vec<f64>);
}

pub fn update_bindings<T: Bound + Component>(
    mut binding_query: Query<&mut T>,
    var_query: Query<&Variable>,
) {
    for mut bound in binding_query.iter_mut() {
        let bindings = bound.get_bindings();
        let values: Vec<f64> = bindings
            .iter()
            .map(|w| var_query.get(*w).unwrap().value())
            .collect();
        bound.set_bindings(values);
    }
}
