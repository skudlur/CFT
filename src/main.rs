use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new();

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    println!("{:?}", netlist_wires);
}

