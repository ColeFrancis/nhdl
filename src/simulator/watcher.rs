use crate::core::types::{EntityId, Logic};

pub struct Watcher {
    pub entity: EntityId,
    pub outputs: Vec<Logic>,
}

impl Watcher {
    pub fn new(entity: EntityId) -> Self {
        Self {
            entity,
            outputs: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.outputs.clear();
    }
}