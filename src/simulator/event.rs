use crate::core::types::NetId;
use crate::core::types::Logic;

pub struct Event {
    pub time: usize,
    pub net: NetId,
    pub new_value: Logic,
}