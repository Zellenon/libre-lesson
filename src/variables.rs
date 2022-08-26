use bevy::{prelude::*, utils::HashMap};
use std::{hash::Hash, sync::Arc};

#[derive(Clone)]
pub struct VariableList(HashMap<String, Variable>);

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
            let mut a: HashMap<String, Variable> = HashMap::new();
            a.insert("0".into(), Variable::Independent(0.));
            a.insert("1".into(), Variable::Independent(1.));
            a
        })
    }

    pub fn get(&self, key: String) -> f64 {
        let var = {
            let x = &self.0;
            x.get(&key).unwrap().clone()
        };
        match var {
            Variable::Independent(x) => x,
            Variable::Dependent(f) => f(&self),
        }
    }

    pub fn insert(&mut self, key: String, value: Variable) {
        (*self).0.insert(key, value);
    }
}
