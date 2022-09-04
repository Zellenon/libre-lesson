use bevy::{prelude::*, utils::hashbrown::HashMap};
use std::sync::Arc;

use super::lambda::{Lam, Num};

type VarQuery<'a> = Query<'a, 'a, (Entity, &'a Variable, &'a Name)>;

#[derive(Clone, Component)]
pub struct Independent;
#[derive(Clone, Component)]
pub struct Dependent;

#[derive(Clone, Component)]
pub enum Variable {
    Independent {
        value: f64,
    },
    Dependent {
        value: f64,
        recalculated: bool,
        // equation: Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>,
        equation: Arc<dyn Lam>,
        // parent: VariableList,
    },
}

impl Variable {
    pub fn recalculated(&self) -> bool {
        if let Variable::Dependent {
            value: _,
            recalculated: r,
            equation: _,
        } = self
        {
            return *r;
        } else {
            return true;
        }
    }

    pub fn set_recalculated(&mut self, is_recalculated: bool) {
        if let Variable::Dependent {
            value: v,
            recalculated: r,
            equation: e,
        } = self
        {
            *r = is_recalculated;
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Variable::Independent { value } => *value,
            Variable::Dependent {
                value,
                recalculated: _,
                equation: _,
            } => *value,
        }
    }
    pub fn set_value(&mut self, new_value: f64) {
        match self {
            Variable::Independent { value } => *value = new_value,
            Variable::Dependent {
                value,
                recalculated: _,
                equation: _,
            } => *value = new_value,
        }
    }

    pub fn equation(&self) -> Arc<dyn Lam> {
        match self {
            Variable::Independent { value } => (Arc::new(Num(*value)) as Arc<dyn Lam>),
            Variable::Dependent {
                value: _,
                recalculated: _,
                equation,
            } => equation.clone(),
        }
    }

    pub fn children(&self) -> Vec<Entity> {
        match self {
            Variable::Independent { value: _ } => Vec::new(),
            Variable::Dependent {
                value: _,
                recalculated: _,
                equation,
            } => equation.children(),
        }
    }

    pub fn calculate(&mut self, context: &HashMap<Entity, &Variable>) {
        self.set_recalculated(true);
        self.set_value(self.equation().get(context));
    }
}

#[derive(Bundle)]
pub struct VariableBundle {
    variable: Variable,
    name: Name,
}

// pub fn build_variables<const DLEN: usize, const ILEN: usize>(
//     mut commands: Commands,
//     independent_vars: [(&str, f64); ILEN],
//     dependent_vars: [(&str, Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>); DLEN],
// ) -> Arc<dyn Fn(&str) -> &Entity> {
//     let mut vars = HashMap::new();
//     for (name, var) in independent_vars {
//         vars.insert(
//             name,
//             commands
//                 .spawn_bundle(VariableBundle {
//                     variable: Variable::Independent { value: var },
//                     name: Name::new(name),
//                 })
//                 .id(),
//         );
//     }
//     for (name, var) in dependent_vars {
//         vars.insert(
//             name,
//             commands
//                 .spawn_bundle(VariableBundle {
//                     variable: Variable::Dependent {
//                         value: -1.,
//                         recalculated: false,
//                         equation: var,
//                     },
//                     name: Name::new(name),
//                 })
//                 .id(),
//         );
//     }
//     return Arc::new(|name: &str| -> &Entity { vars.get(name).unwrap() });
// }

pub fn dependent<T: Lam + 'static>(
    commands: &mut Commands,
    name: &'static str,
    equation: T,
) -> Entity {
    commands
        .spawn()
        .insert(Name::new(name))
        .insert(Variable::Dependent {
            value: 0.,
            recalculated: false,
            equation: Arc::new(equation),
        })
        .insert(Dependent)
        .id()
}

pub fn independent(commands: &mut Commands, name: &'static str, value: f64) -> Entity {
    commands
        .spawn()
        .insert(Name::new(name))
        .insert(Variable::Independent { value })
        .insert(Independent)
        .id()
}