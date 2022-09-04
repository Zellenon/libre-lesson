use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use super::variable::Variable;

pub struct VariableList<'a> {
    variables: HashMap<Entity, &'a Variable>, // Stores ids of Variables
    names: HashMap<String, Entity>,
}

impl<'a> VariableList<'a> {
    pub fn new() -> Self {
        VariableList {
            variables: HashMap::new(),
            names: HashMap::new(),
        }
    }
    pub fn get<T: Into<String>>(&self, key: T) -> f64 {
        self.get_value(self.get_entity(key))
    }

    pub fn get_value(&self, key: &Entity) -> f64 {
        self.variables.get(key).unwrap().value()
    }

    pub fn get_entity<T: Into<String>>(&self, key: T) -> &Entity {
        self.names.get(&key.into()).unwrap()
    }

    // pub fn insert<T: Into<String> + Clone>(&mut self, key: T, value: &Variable) {
    //     (*self).variables.insert(key.into(), *value);
    // }

    pub fn from_query(query: &'a Query<(Entity, &Variable, &Name)>) -> Self {
        let mut names = HashMap::new();
        let mut variables = HashMap::new();
        for (e, var, name) in query.iter() {
            names.insert(name.into(), e);
            variables.insert(e, var);
        }
        Self { variables, names }
    }
}
