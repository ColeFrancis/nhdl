use crate::core::types::Logic;
use crate::circuit::gate::Gate;
use crate::circuit::circuit::Circuit;

fn eval_nand(a: Logic, b: Logic) -> Logic {
    match (a, b) {
        (Logic::ON, Logic::ON) => Logic::OFF,
        (Logic::OFF, _) | (_, Logic::OFF) => Logic::ON,
        _ => Logic::X,
    }
}

pub fn eval_gate(circuit: &Circuit, gate: &Gate) -> Logic {
    let a = circuit.nets[gate.a].value;
    let b = circuit.nets[gate.b].value;

    eval_nand(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_basic() {
        let on_on = eval_nand(Logic::ON, Logic::ON);

        assert_eq!(on_on, Logic::OFF, "Received {:?} for ON, ON", on_on);

        let on_off = eval_nand(Logic::ON, Logic::OFF);

        assert_eq!(on_off, Logic::ON, "Received {:?} for ON, OFF", on_off);

        let off_on = eval_nand(Logic::OFF, Logic::ON);

        assert_eq!(off_on, Logic::ON, "Received {:?} for OFF, ON", off_on);

        let off_off = eval_nand(Logic::OFF, Logic::OFF);

        assert_eq!(off_off, Logic::ON, "Received {:?} for OFF, OFF", off_off);
    }#[test]

    fn test_nand_unknown() {
        let x_x = eval_nand(Logic::X, Logic::X);

        assert_eq!(x_x, Logic::X, "Received {:?} for X, X", x_x);

        let x_on = eval_nand(Logic::X, Logic::ON);

        assert_eq!(x_on, Logic::X, "Received {:?} for X, ON", x_on);

        let x_off = eval_nand(Logic::X, Logic::OFF);

        assert_eq!(x_off, Logic::ON, "Received {:?} for X, OFF", x_off);
    }
}