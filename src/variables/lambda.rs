use bevy::prelude::*;

use super::variable::Variable;

pub trait Lam: Send + Sync {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64;
    fn children(&self) -> Vec<Entity>;
}

pub struct Add<T: Lam, U: Lam>(pub T, pub U);
impl<T: Lam, U: Lam> Lam for Add<T, U> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context) + self.1.get(context)
    }

    fn children(&self) -> Vec<Entity> {
        let mut temp = self.0.children();
        temp.append(&mut self.1.children().clone());
        temp
    }
}

pub struct Sub<T: Lam, U: Lam>(pub T, pub U);
impl<T: Lam, U: Lam> Lam for Sub<T, U> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context) - self.1.get(context)
    }

    fn children(&self) -> Vec<Entity> {
        let mut temp = self.0.children();
        temp.append(&mut self.1.children().clone());
        temp
    }
}

pub struct Mul<T: Lam, U: Lam>(pub T, pub U);
impl<T: Lam, U: Lam> Lam for Mul<T, U> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context) * self.1.get(context)
    }

    fn children(&self) -> Vec<Entity> {
        let mut temp = self.0.children();
        temp.append(&mut self.1.children().clone());
        temp
    }
}

pub struct Div<T: Lam, U: Lam>(pub T, pub U);
impl<T: Lam, U: Lam> Lam for Div<T, U> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context) / self.1.get(context)
    }

    fn children(&self) -> Vec<Entity> {
        let mut temp = self.0.children();
        temp.append(&mut self.1.children().clone());
        temp
    }
}

pub struct Mod<T: Lam, U: Lam>(pub T, pub U);
impl<T: Lam, U: Lam> Lam for Mod<T, U> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context) % self.1.get(context)
    }

    fn children(&self) -> Vec<Entity> {
        let mut temp = self.0.children();
        temp.append(&mut self.1.children().clone());
        temp
    }
}

pub struct Sin<T: Lam>(pub T);
impl<T: Lam> Lam for Sin<T> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context).sin()
    }

    fn children(&self) -> Vec<Entity> {
        self.0.children()
    }
}

pub struct Cos<T: Lam>(pub T);
impl<T: Lam> Lam for Cos<T> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context).cos()
    }

    fn children(&self) -> Vec<Entity> {
        self.0.children()
    }
}

pub struct Tan<T: Lam>(pub T);
impl<T: Lam> Lam for Tan<T> {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0.get(context).tan()
    }

    fn children(&self) -> Vec<Entity> {
        self.0.children()
    }
}

pub struct Var(pub Entity);
impl Lam for Var {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        context
            .iter()
            .filter(|w| w.0 == self.0)
            .next()
            .unwrap()
            .1
            .value()
    }

    fn children(&self) -> Vec<Entity> {
        vec![self.0]
    }
}

pub struct Num(pub f64);
impl Lam for Num {
    fn get(&self, _context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        self.0
    }

    fn children(&self) -> Vec<Entity> {
        Vec::new()
    }
}

pub struct Sum(pub Vec<Entity>);
impl Lam for Sum {
    fn get(&self, context: &Vec<&mut (Entity, Mut<Variable>)>) -> f64 {
        0. + self
            .0
            .iter()
            .map(|e| {
                context
                    .iter()
                    .filter(|w| w.0 == *e)
                    .next()
                    .unwrap()
                    .1
                    .value()
            })
            .sum::<f64>()
    }

    fn children(&self) -> Vec<Entity> {
        self.0.clone()
    }
}
impl Sum {
    pub fn add(&mut self, new_entry: Entity) {
        self.0.push(new_entry);
    }
    pub fn remove(&mut self, entity: Entity) {
        self.0.retain(|&w| w != entity);
    }
}
