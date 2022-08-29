use bevy::{prelude::*, utils::HashMap};
use std::ops::Add;
use std::sync::Arc;

#[derive(Component, Clone)]
pub struct VariableList {
    variables: HashMap<String, Variable>,
    children: HashMap<String, VariableList>,
}

#[derive(Clone)]
pub enum Variable {
    Independent {
        value: f64,
    },
    Dependent {
        value: f64,
        recalculated: bool,
        equation: Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>,
    },
}

impl Variable {
    pub fn independent(value: f64) -> Variable {
        Variable::Independent { value }
    }

    pub fn dependent(equation: Arc<dyn Fn(&VariableList) -> f64 + Send + Sync>) -> Variable {
        Variable::Dependent {
            value: -1.,
            recalculated: false,
            equation,
        }
    }
}

impl VariableList {
    pub fn new() -> Self {
        VariableList {
            variables: {
                let mut a: HashMap<String, Variable> = HashMap::new();
                a.insert("0".into(), Variable::independent(0.));
                a.insert("1".into(), Variable::independent(1.));
                a
            },
            children: HashMap::new(),
        }
    }

    pub fn get<T: Into<String>>(&self, key: T) -> f64 {
        let key_string: String = key.into();
        let mut parts = key_string.split('.').collect::<Vec<&str>>();
        if parts.len() > 1 {
            let child_key: String = (*parts.first().unwrap()).into();
            parts.remove(0);
            let child = self.get_child(child_key);
            (*child).get(parts.join("."))
        } else {
            let var = &self.variables.get(&key_string).unwrap().clone();
            match var {
                Variable::Independent { value } => *value,
                Variable::Dependent {
                    value,
                    recalculated,
                    equation,
                } => *value,
            }
        }
    }

    pub fn insert<T: Into<String>>(&mut self, key: T, value: Variable) {
        (*self).variables.insert(key.into(), value);
    }

    pub fn add_child<T: Into<String>>(&mut self, key: T, child: VariableList) {
        (*self).children.insert(key.into(), child);
    }

    fn get_child<T: Into<String>>(&self, key: T) -> &VariableList {
        (*self).children.get(&key.into()).unwrap()
    }
}

pub fn variable_devaluate(var_query: Query<&VariableList>) {}

pub fn variable_update_system(var_query: Query<&VariableList>) {}

impl Add for VariableList {
    type Output = VariableList;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_map = VariableList::new();
        new_map.variables.extend(self.variables);
        new_map.variables.extend(rhs.variables);
        new_map
    }
}
