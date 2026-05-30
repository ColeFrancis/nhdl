use super::gate::Gate;
use super::net::Net;

pub struct Circuit {
    pub gates: Vec<Gate>,
    pub nets: Vec<Net>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            nets: Vec::new(),
        }
    }
}