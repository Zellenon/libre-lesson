use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

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
}
