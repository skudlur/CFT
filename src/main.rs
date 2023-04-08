use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn fault_equivalence_op(gate: &str, out: &Vec, in_pref: &Vec) {
    let mut fault_to_remove = Vec::new();
    let mut s0_out = String::new();
    let mut s1_out = String::new();
    let mut s0_in1 = String::new();

    if(gate == "AND") {
        s0_out = format!("sa0_{}", out);
        s1_out = format!("sa1_{}", out);
        s1_in1 = format!("sa0_{}", in_pref);
    }
}

fn main() {
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new();

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    println!("{:?}", netlist_wires);

    let mut fault_vectors = Vec::new();
    let mut fault_values = Vec::new();
    let mut value = 0;

    for i in 1..netlist_wires.len()+1 {
        let sa0 = format!("sao_{}",i);
        let sa1 = format!("sa1_{}",i);

        fault_vectors.push(sa0);
        fault_vectors.push(sa1);
    }
    println!("{:?}", fault_vectors); 
    while value <= (fault_vectors.len()-1) {
        fault_values.push(1);
        value += 1;
    }

    let mut fault_map = BTreeMap::new();

    for (i, (&ref fault_vector, &fault_value)) in fault_vectors.iter().zip(fault_values.iter()).enumerate() {
        fault_map.insert(i, (fault_vector, fault_value));
    }
    println!("{:?}", fault_map);
}
