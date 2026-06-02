use super::relation::Relation;
use super::entity::Entity;

pub struct Network {
    pub relations: Vec<Relation>,
    pub entities: Vec<Entity>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            relations: Vec::new(),
            entities: Vec::new(),
        }
    }
}