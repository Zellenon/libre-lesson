use bevy::{prelude::*, utils::HashMap};
use num_traits::Num;
use std::{hash::Hash, sync::Arc};

pub trait MathVar: Hash + Eq + Clone {}

#[derive(Clone)]
pub struct VariableList<T: MathVar>(HashMap<T, Variable<T>>);

#[derive(Clone)]
pub enum Variable<T: MathVar> {
    Independent(f64),
    Dependent(Arc<dyn Fn(&VariableList<T>) -> f64>),
}

macro_rules! dependent {
    ($t: ty, $e:expr) => {
        Variable::Dependent(Arc::new(move |vars: &VariableList<$t>| return $e))
    };
}

impl<T: MathVar> VariableList<T> {
    pub fn new() -> Self {
        VariableList(HashMap::new())
    }

    pub fn get(&self, key: T) -> f64 {
        let var = {
            let x = &self.0;
            x.get(&key).unwrap().clone()
        };
        match var {
            Variable::Independent(x) => x,
            Variable::Dependent(f) => f(&self),
        }
    }

    pub fn insert(&mut self, key: T, value: Variable<T>) {
        (*self).0.insert(key, value);
    }
}

#[derive(Component, Clone, Copy)]
pub struct OldVariable<T: Num>(pub T);

impl From<f64> for OldVariable<f64> {
    fn from(x: f64) -> Self {
        OldVariable(x)
    }
}

#[derive(Bundle)]
pub struct VariableBundle {
    pub name: Name,
    pub value: OldVariable<f64>,
}

impl VariableBundle {
    pub fn new(name: String, value: f64) -> Self {
        Self {
            name: Name::new(name),
            value: OldVariable(value),
        }
    }
}

impl Default for VariableBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Unnamed Variable"),
            value: OldVariable(1.),
        }
    }
}
