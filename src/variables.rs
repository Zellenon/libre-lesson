use bevy::{prelude::*, utils::HashMap};
use std::ops::Add;
use std::{hash::Hash, sync::Arc};

#[derive(Component, Clone)]
pub struct VariableList {
    variables: HashMap<String, Variable>,
    children: HashMap<String, VariableList>,
}

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
        VariableList {
            variables: {
                let mut a: HashMap<String, Variable> = HashMap::new();
                a.insert("0".into(), Variable::Independent(0.));
                a.insert("1".into(), Variable::Independent(1.));
                a
            },
            children: HashMap::new(),
        }
    }

    pub fn get<T: Into<String>>(&self, key: T) -> f64 {
        let var = { &self.variables.get(&key.into()).unwrap().clone() };
        match var {
            Variable::Independent(x) => *x,
            Variable::Dependent(f) => f(&self),
        }
    }

    pub fn insert<T: Into<String>>(&mut self, key: T, value: Variable) {
        (*self).variables.insert(key.into(), value);
    }

    pub fn add_child<T: Into<String>>(&mut self, key: T, child: VariableList) {
        (*self).children.insert(key.into(), child);
    }
}

impl Add for VariableList {
    type Output = VariableList;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_map = VariableList::new();
        new_map.variables.extend(self.variables);
        new_map.variables.extend(rhs.variables);
        new_map
    }
}
