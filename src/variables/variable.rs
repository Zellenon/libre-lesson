use bevy::prelude::*;
use std::sync::Arc;

use super::list::VariableList;

#[derive(Clone, Component)]
pub enum Variable {
    Independent {
        value: f64,
    },
    Dependent {
        value: f64,
        recalculated: bool,
        equation: Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>,
        // parent: VariableList,
    },
}

impl Variable {
    pub fn recalculated(&self) -> bool {
        if let Variable::Dependent {
            value: v,
            recalculated: r,
            equation: e,
        } = self
        {
            return *r;
        } else {
            return true;
        }
    }

    pub fn set_recalculated(&self, is_recalculated: bool) {
        if let Variable::Dependent {
            value: v,
            recalculated: r,
            equation: e,
        } = self
        {
            *self = Variable::Dependent {
                value: *v,
                recalculated: is_recalculated,
                equation: *e,
            };
        }
    }
}

#[derive(Bundle)]
pub struct VariableBundle {
    variable: Variable,
    name: Name,
}

pub fn build_variables<T: Into<String>, const DLEN: usize, const ILEN: usize>(
    mut commands: Commands,
    independent_vars: [(T, f64); ILEN],
    dependent_vars: [(T, Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>); DLEN],
) {
    for (name, var) in independent_vars {
        commands.spawn_bundle(VariableBundle {
            variable: Variable::Independent { value: var },
            name: Name::new(name.into()),
        });
    }
    for (name, var) in dependent_vars {
        commands.spawn_bundle(VariableBundle {
            variable: Variable::Dependent {
                value: -1.,
                recalculated: false,
                equation: var,
            },
            name: Name::new(name.into()),
        });
    }
}
