use crate::core::types::EntityId;
use crate::core::types::Logic;

pub struct Event {
    pub time: usize,
    pub entity: EntityId,
    pub new_value: Logic,
}