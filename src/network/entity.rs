use crate::core::types::Logic;
use crate::core::types::RelationId;

pub struct Entity {
    pub value: Logic,
    pub sinks: Vec<RelationId>,
}