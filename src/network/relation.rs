use crate::core::types::EntityId;

pub enum RelationKind {
    NAND,
}

pub struct Relation {
    pub kind: RelationKind,
    pub delay: u8,
    pub a: EntityId,
    pub b: EntityId,
    pub out: EntityId,
}

impl Relation {
    pub fn new(a: EntityId, b: EntityId, out: EntityId) -> Self {
        Self {
            kind: RelationKind::NAND,
            delay: 1,
            a, 
            b, 
            out,
        }
    }
}