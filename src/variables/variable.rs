use bevy::prelude::*;
use std::sync::Arc;

use super::{
    group::Group,
    lambda::{Lam, Num},
};

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
        equation: Arc<dyn Lam>,
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
            value: _v,
            recalculated: r,
            equation: _e,
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

    pub fn calculate(&mut self, context: &Vec<&mut (Entity, Mut<Variable>)>) {
        self.set_recalculated(true);
        self.set_value(self.equation().get(context));
    }
}

#[derive(Bundle)]
pub struct VariableBundle {
    variable: Variable,
    name: Name,
}

pub fn dependent<T: Lam + 'static>(
    commands: &mut Commands,
    group: &Group,
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
        .insert(group.clone())
        .id()
}

pub fn independent(
    commands: &mut Commands,
    group: &Group,
    name: &'static str,
    value: f64,
) -> Entity {
    commands
        .spawn()
        .insert(Name::new(name))
        .insert(Variable::Independent { value })
        .insert(Independent)
        .insert(group.clone())
        .id()
}
