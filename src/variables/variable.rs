use bevy::{prelude::*, utils::hashbrown::HashMap};
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

    pub fn value(self) -> f64 {
        match self {
            Variable::Independent { value } => value,
            Variable::Dependent {
                value,
                recalculated,
                equation,
            } => value,
        }
    }
}

#[derive(Bundle)]
pub struct VariableBundle {
    variable: Variable,
    name: Name,
}

pub fn build_variables<const DLEN: usize, const ILEN: usize>(
    mut commands: Commands,
    independent_vars: [(&str, f64); ILEN],
    dependent_vars: [(&str, Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>); DLEN],
) -> Arc<dyn Fn(&str) -> &Entity> {
    let mut vars = HashMap::new();
    for (name, var) in independent_vars {
        vars.insert(
            name,
            commands
                .spawn_bundle(VariableBundle {
                    variable: Variable::Independent { value: var },
                    name: Name::new(name),
                })
                .id(),
        );
    }
    for (name, var) in dependent_vars {
        vars.insert(
            name,
            commands
                .spawn_bundle(VariableBundle {
                    variable: Variable::Dependent {
                        value: -1.,
                        recalculated: false,
                        equation: var,
                    },
                    name: Name::new(name),
                })
                .id(),
        );
    }
    return Arc::new(|name: &str| -> &Entity { vars.get(name).unwrap() });
}
