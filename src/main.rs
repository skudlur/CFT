use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn fault_equivalence_op<T>(gate: &str, out: &str, in_pref: &str) {
    let mut fault_to_remove: Vec<T> = Vec::new();

    if gate == "AND" {
        let s0_out = format!("sa0_{}", out);
        let s1_out = format!("sa1_{}", out);
        let s0_in1 = format!("sa0_{}", in_pref);
    }
}

fn stuck_at_fault_number(num_nets: &usize) -> usize{
    num_nets * 2
}

fn main() {
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new();

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    //println!("{:?}", netlist_wires);

    let split_netlist: Vec<Vec<_>> = netlist_wires.iter().map(|s| s.split(" ").collect()).collect();

    //println!("{:?}", split_netlist);

    let mut nets = Vec::new();

    for i in 0..netlist_wires.len() {
        let temp_slice = &split_netlist[i][1..4];
        nets.push(temp_slice);
    }

    let mut combined_nets: Vec<_> = nets.concat();
    combined_nets.sort();
    combined_nets.dedup();

    println!("Nets: {:?}", combined_nets);
    
    let temp_num: usize = combined_nets.len();
    let mut num_fault: usize = 0;
    num_fault = stuck_at_fault_number(&temp_num);
    println!("Number of total stuck-at faults: {}", num_fault);

    let mut fault_vectors = Vec::new();
    let mut fault_values = Vec::new();
    let mut value = 0;

    for i in 1..combined_nets.len()+1 {
        let sa0 = format!("sao_{}",i);
        let sa1 = format!("sa1_{}",i);

        fault_vectors.push(sa0);
        fault_vectors.push(sa1);
    }
    println!("Stuck-at faults: {:?}", fault_vectors); 
    while value <= (fault_vectors.len()-1) {
        fault_values.push(1);
        value += 1;
    }

    let mut fault_map = BTreeMap::new();

    for (i, (&ref fault_vector, &fault_value)) in fault_vectors.iter().zip(fault_values.iter()).enumerate() {
        fault_map.insert(i+1, (fault_vector, fault_value));
    }
    println!("Initial fault values: {:?}", fault_map);
}
