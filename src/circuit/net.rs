use crate::core::types::Logic;
use crate::core::types::GateId;

pub struct Net {
    pub value: Logic,
    pub sinks: Vec<GateId>,
}