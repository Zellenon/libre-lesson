use bevy::prelude::*;
use std::sync::Arc;

#[derive(Clone, Component)]
pub enum Variable {
    Independent {
        value: f64,
    },
    Dependent {
        value: f64,
        recalculated: bool,
        equation: Arc<dyn Fn(&Query<(Entity, &Variable)>) -> f64 + Send + Sync>,
        // parent: VariableList,
    },
}
