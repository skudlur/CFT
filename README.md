# CFT
Circuit Fault Tester

### What is CFT?
Circuit Fault Tester is a tool used to parse a netlist file and perform fault optimizations. This tool is developed as a part of my elective course - "Testing of Digital VLSI Circuits". **It is an educational tool.**

## Installation
- Install the Rust compiler 'rustc' using rustup.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Clone this repo 

```bash
git clone https://github.com/suhaskv1/CFT.git
```
- Change directory to CFT/src and run the following

```bash
cd CFT/src
cargo run
```
- Change the netlist as required 

### Checklist
1. ~Fault equivalence operation with collapse ratio.~
2. ~Fault dominance operation with collapse ratio.~
3. D-Algorithm
