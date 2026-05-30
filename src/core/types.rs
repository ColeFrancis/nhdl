pub type NetId = usize;
pub type GateId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Logic {
    ON,
    OFF,
    X,
}