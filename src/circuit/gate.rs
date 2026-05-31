use crate::core::types::NetId;

pub enum GateType {
    NAND,
}

pub struct Gate {
    pub gate_type: GateType,
    pub delay: u8,
    pub a: NetId,
    pub b: NetId,
    pub out: NetId,
}

impl Gate {
    pub fn new(a: NetId, b: NetId, out: NetId) -> Self {
        Self {
            gate_type: GateType::NAND,
            delay: 1,
            a, 
            b, 
            out,
        }
    }
}