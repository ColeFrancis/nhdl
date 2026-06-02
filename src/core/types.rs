pub type EntityId = usize;
pub type RelationId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Logic {
    ON,
    OFF,
    X,
}