mod core_structs;
mod core_logic;
mod sim_logic;

use core_structs::*;
use sim_logic::*;


fn main() {
    let mut circuit = Circuit::new();

    circuit.gates.push(Gate {
        a: 0,
        b: 1,
        out: 2,
    });

    circuit.nets.push(Net {
        value: Logic::X,
        sinks: vec![0],
    });

    circuit.nets.push(Net {
        value: Logic::X,
        sinks: vec![0],
    });

    circuit.nets.push(Net {
        value: Logic::X,
        sinks: vec![],
    });

    let mut simulation = Simulation::new(circuit);

    simulation.init_net(0, Logic::OFF);

    simulation.init_net(1, Logic::OFF);

    simulation.run();

    let output1 = simulation.read_net(2);

    simulation.reset();

    simulation.init_net(0, Logic::OFF);

    simulation.init_net(1, Logic::ON);

    simulation.run();

    let output2 = simulation.read_net(2);

    simulation.reset();

    simulation.init_net(0, Logic::ON);

    simulation.init_net(1, Logic::ON);

    simulation.run();

    let output3 = simulation.read_net(2);

    println!("OFF, OFF: {:?}", output1);
    println!("OFF, ON: {:?}", output2);
    println!("ON, ON: {:?}", output3);
}
