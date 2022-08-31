use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use std::sync::Arc;

use super::variable::Variable;

#[derive(Component, Clone)]
pub struct VariableGroup {
    variables: HashMap<String, Entity>, // Stores ids of Variables
    children: HashMap<String, Entity>,  // Stores ids of VariableList
}

impl VariableGroup {
    pub fn new() -> Self {
        VariableGroup {
            variables: HashMap::new(),
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
                    parent,
                } => *value,
            }
        }
    }

    pub fn insert<T: Into<String> + Clone>(&mut self, key: T, value: Variable) {
        (*self).variables.insert(key.into(), value);
    }

    // pub fn add_child<T: Into<String>>(&mut self, key: T, child: Entity) {
    //     (*self).children.insert(key.into(), child);
    // }

    // fn get_child<T: Into<String>>(&self, key: T) -> &VariableGroup {
    //     (*self).children.get(&key.into()).unwrap()
    // }

    //     pub fn independent<T: Into<String> + Clone>(&mut self, name: T, value: f64) -> Variable {
    //         let var = Variable::Independent { value };
    //         self.insert(name, var);
    //         var
    //     }

    //     pub fn dependent<T: Into<String> + Clone>(
    //         &mut self,
    //         name: T,
    //         equation: Arc<dyn Fn(&VariableGroup) -> f64 + Send + Sync>,
    //     ) -> Variable {
    //         let var = Variable::Dependent {
    //             value: -1.,
    //             recalculated: false,
    //             equation,
    //             parent: *self,
    //         };
    //         self.insert(name, var);
    //         var
    //     }
}

// impl Add for VariableGroup {
//     type Output = VariableGroup;

//     fn add(self, rhs: Self) -> Self::Output {
//         let mut new_map = VariableGroup::new();
//         new_map.variables.extend(self.variables);
//         new_map.variables.extend(rhs.variables);
//         new_map
//     }
// }
