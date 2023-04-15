use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
fn fault_equivalence_op<T>(f_comb_vec: &Vec<T>) {
    /* Function of the fault equivalence operation */
    for i in 0..f_comb_vec.len() {
        let mut serial_val = f_comb_vec[i][0];
        let mut gate_val = f_comb_vec[i][1];
        let mut string_index_val = String::new();

        match gate_val {
            "AND" => {
                f_comb_vec[i][  
            }
            "OR" => {
            
            }
            "NOT" => {
                
            }
            "_" => panic!("Invalid gate!"),
        }
    }
}
*/
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
    serial_no: usize,
    sa0: u32,
    sa1: u32,
    input_source: Vec<String>,
    output_source: Vec<String>,
}

#[derive(Default, Debug)]
struct Gate {
    serial_no: usize,
    val: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

fn main() {
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new(); // Vector for netlist lines

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    println!("{:?}", netlist_wires);

    let split_netlist: Vec<Vec<_>> = netlist_wires.iter().map(|s| s.split(" ").collect()).collect(); // Vector to split the netlist line

    println!("{:?}", split_netlist);

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
        
        let mut temp_input_wire = Vec::new();
        temp_input_wire.push(temp_input_1.to_string());
        temp_input_wire.push(temp_input_2.to_string());
        let mut temp_output_wire = Vec::new(); 
        temp_output_wire.push(temp_output.to_string());
        
        let temp_wire = Wire {
            serial_no: i+1,
            sa0: 1,
            sa1: 1,
            input_source: temp_input_wire.clone(),
            output_source: temp_output_wire.clone(),
        };
        wires.push(temp_wire);

        let temp_gate = Gate {
            serial_no: i+1,
            val: temp_gate,
            inputs: temp_input_wire,
            outputs: temp_output_wire,
        };
        gates.push(temp_gate);
    }

    println!("{:?}", wires);
    println!("{:?}", gates);
}
