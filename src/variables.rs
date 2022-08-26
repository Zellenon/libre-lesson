use bevy::{prelude::*, utils::HashMap};
use std::{hash::Hash, sync::Arc};

#[derive(Clone)]
pub struct VariableList(HashMap<&'static str, Variable>);

#[derive(Clone)]
pub enum Variable {
    Independent(f64),
    Dependent(Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>),
}

macro_rules! dependent {
    ($t: ty, $e:expr) => {
        Variable::Dependent(Arc::new(move |vars: &VariableList<$t>| return $e))
    };
}

impl VariableList {
    pub fn new() -> Self {
        VariableList({
            let mut a = HashMap::new();
            a.insert("0", Variable::Independent(0.));
            a.insert("1", Variable::Independent(1.));
            a
        })
    }

    pub fn get(&self, key: &str) -> f64 {
        let var = {
            let x = &self.0;
            x.get(&key).unwrap().clone()
        };
        match var {
            Variable::Independent(x) => x,
            Variable::Dependent(f) => f(&self),
        }
    }

    pub fn insert(&mut self, key: &'static str, value: Variable) {
        (*self).0.insert(key, value);
    }
}
