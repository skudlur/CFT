use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn fault_equivalence_op<T>() {
    /* Function of the fault equivalence operation */
    let mut fault_to_collapse: Vec<T> = Vec::new();

}

/*
fn split_btreemap<K, V>(bmap: &BTreeMap<K, V>) {
    /* Returns split BTreeMaps of the faults of a particular net */
    let mut sliced_map = BTreeMap::new();
    let mut bmap_iter = bmap.iter().take(3);
    while let Some((k,v)) = bmap_iter.next() {
        sliced_map.insert(*k, *v);
    }
}
*/

fn stuck_at_fault_number(num_nets: &usize) -> usize{
    /* Function to return number of stuck-at faults */
    num_nets * 2
}

fn main() {
    let netlist_file = File::open("netlist.txt").unwrap();
    let reader = BufReader::new(netlist_file);

    let mut netlist_wires = Vec::new(); // Vector for netlist lines

    for line in reader.lines() {
        netlist_wires.push(line.unwrap());
    }
    //println!("{:?}", netlist_wires);

    let split_netlist: Vec<Vec<_>> = netlist_wires.iter().map(|s| s.split(" ").collect()).collect(); // Vector to split the netlist line

    //println!("{:?}", split_netlist);

    let mut nets = Vec::new(); // Vector of nets

    for i in 0..netlist_wires.len() {
        let temp_slice = &split_netlist[i][1..4];
        nets.push(temp_slice);
    }

    let mut combined_nets: Vec<_> = nets.concat(); // Vector with all nets
    combined_nets.sort();
    combined_nets.dedup();

    println!("Nets: {:?}", combined_nets);
    
    let temp_num: usize = combined_nets.len();
    let mut num_fault: usize = 0;
    num_fault = stuck_at_fault_number(&temp_num);
    println!("Number of total stuck-at faults: {}", num_fault);

    let mut fault_vectors = Vec::new(); // Vector with stuck-at faults
    let mut fault_values = Vec::new(); // Vector with stuck-at values for the fault 
                                       // (1 -> Present, 0 -> Collapsed/Absent)
    let mut value = 0;

    for i in 1..combined_nets.len()+1 {
        let sa0 = format!("sao_{}",i);
        let sa1 = format!("sa1_{}",i);

        fault_vectors.push(sa0);
        fault_vectors.push(sa1);
    }

    println!("Stuck-at faults: {:?}", fault_vectors); 
    
    while value <= (fault_vectors.len()-1) {
        fault_values.push("1");
        value += 1;
    }

    //println!("{:?}", fault_values);

    let mut nets_doubled = Vec::new(); // Vector to hold repeated values 

    for i in 0..combined_nets.len() {
        nets_doubled.push(combined_nets[i]);
        nets_doubled.push(combined_nets[i]);
    }

    //println!("{:?}", nets_doubled);

    let mut fault_combined_vec: Vec::<(String, String, String)> = nets_doubled.into_iter()
        .zip(fault_vectors.into_iter())
        .zip(fault_values.into_iter())
        .map(|((x,y), z)| (x.to_string(), y.to_string(), z.to_string()))
        .collect(); // Combined vector with net, stuck-at fault and value
    
    //println!("{:?}", fault_combined_vec);

    let mut fault_map = BTreeMap::new();

    for (i, x) in fault_combined_vec.iter().enumerate() {
        fault_map.insert(i+1, x.to_owned());
    }

    println!("{:?}", fault_map);
}
