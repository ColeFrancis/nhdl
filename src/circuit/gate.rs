use crate::core::types::NetId;

pub struct Gate {
    pub a: NetId,
    pub b: NetId,
    pub out: NetId,
}