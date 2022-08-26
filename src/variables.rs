use bevy::{prelude::*, utils::HashMap};
use num_traits::Num;
use std::{hash::Hash, sync::Arc};

pub trait MathVar: Hash + Eq + Clone + Sync + Send {}

#[derive(Clone, Copy)]
pub struct VariableList<T: MathVar>(HashMap<T, Variable<T>>);

#[derive(Clone)]
pub enum Variable<T: MathVar> {
    Independent(f64),
    Dependent(Arc<dyn Fn(&VariableList<T>) -> f64 + Send + Sync>),
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

    pub fn get_raw(&self, key: T) -> &Variable<T> {
        (self).0.get(&key).unwrap()
    }

    pub fn insert(&mut self, key: T, value: Variable<T>) {
        (*self).0.insert(key, value);
    }
}
