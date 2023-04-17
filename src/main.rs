use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fault_equivalence_op(wire_vec: &mut Vec<Wire>, gate_vec: &mut Vec<Gate>) {
    /* Function of the fault equivalence operation */
    for i in 0..wire_vec.len() {
        if wire_vec[i].gate_assoc == "AND" {
            for j in 0..gate_vec.len() {
                if wire_vec[i].gate_assoc == gate_vec[j].val && gate_vec[j].inputs.contains(&wire_vec[i].input_source) {
                    wire_vec[i].sa0 = 0;
                }
            }
            println!("Fault for {} collapsed!", wire_vec[i].gate_assoc);
        }
        else if wire_vec[i].gate_assoc == "OR" {
            for j in 0..gate_vec.len() {
                if wire_vec[i].gate_assoc == gate_vec[j].val && gate_vec[j].inputs.contains(&wire_vec[i].input_source) {
                    wire_vec[i].sa1 = 0;
                }
            }
            println!("Fault for {} collapsed!", wire_vec[i].gate_assoc);
        }
    }
    println!("{:?}", wire_vec);
    println!("{:?}", gate_vec);
}

fn logo_display() {
    /* CFT logo */
    let filename = "logo.txt";
    let logo_con = fs::read_to_string(filename)
        .expect("Failed to read the file");
    println!("{}",logo_con);
}

fn stuck_at_fault_number(num_nets: &usize) -> usize{
    /* Function to return number of stuck-at faults */
    num_nets * 2
}

#[derive(Default, Debug)]
struct Wire {
    wire_no: usize,
    sa0: u32,
    sa1: u32,
    input_source: String,
    gate_assoc: String,
}

#[derive(Default, Debug)]
struct Gate {
    gate_no: usize,
    val: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

fn main() {
    logo_display();
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new(); // Vector for netlist lines

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    //println!("{:?}", netlist_wires);

    let split_netlist: Vec<Vec<_>> = netlist_wires.iter().map(|s| s.split(" ").collect()).collect(); // Vector to split the netlist line

    //println!("{:?}", split_netlist);

    for i in 0..split_netlist.len() {
        if split_netlist[i][0] != "NOT" && split_netlist[i].len() < 4 {
            panic!("Netlist error! Please rectify");
        }
    } 

    let mut wires = Vec::new(); // Vector of wires
    let mut gates = Vec::new(); // Vector of gates

    for i in 0..netlist_wires.len() {
        let temp_gate = &split_netlist[i][0];
        let temp_gate = temp_gate.to_string();
        let temp_input_1 = &split_netlist[i][1];
        let temp_input_2 = &split_netlist[i][2];
        let temp_output = &split_netlist[i][3];
        
        for j in 1..4 {
            let temp_input = &split_netlist[i][j];
            let temp_input = temp_input.to_string();
            let temp_wire = Wire {
                wire_no: j,
                sa0: 1,
                sa1: 1,
                input_source: temp_input,
                gate_assoc: temp_gate.clone(),
            };
            wires.push(temp_wire);
        }

        let mut temp_input_wire = Vec::new();
        temp_input_wire.push(temp_input_1.to_string());
        temp_input_wire.push(temp_input_2.to_string());
        let mut temp_output_wire = Vec::new(); 
        temp_output_wire.push(temp_output.to_string());
        
        let temp_gate = Gate {
            gate_no: i+1,
            val: temp_gate,
            inputs: temp_input_wire,
            outputs: temp_output_wire,
        };
        gates.push(temp_gate);
    }

    println!("{:?}", wires);
    println!("{:?}", gates);


    fault_equivalence_op(&mut wires, &mut gates);
}
