#![allow(warnings)]

use std::fs::File;
use std::io::{self, BufReader, BufRead, Read};
use crate::aig_structure::aig::AIG;
use crate::aig_structure::signal::Signal;

/// struct that builds an aig from an aiger file based on this paper: https://fmv.jku.at/aiger/FORMAT.aiger
/// aig: aig from aiger file.
/// inputs: vector that contains all input signals
/// outputs: vector that contains all input outputs
pub struct AigerReader {
    pub aig: AIG,
    pub inputs: Vec<Signal>,
    pub outputs: Vec<Signal>,
}

impl AigerReader {

    /// read AIG from file
    pub fn from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file); //BufReader reduces the number of system calls by buffering data internally


        // Convert literal → signal
        // But what if we have two literals 4 and 5? Both would have the same index but different outputs?
        // That's actually how it's supposed to be. 4 and 5 represent the same node.
        // That means both shouldn't appear together.
        // Otherwise, we can normalize it: not x and x = 0. See aig.create_and rules.
        
        // The literals are (rhs0, rhs1 und lhs). 
        // input literals = 2, 4, 6, .... 2*i --> input variable indices = literal / 2 or 1, 2, 3, .... i
        // and literal = 2*(I+L)+2, 4*(I+L)+4, .... 2*(I+L+A)   --> and variable indices = i + l + 1  or lhs / 2, da lhs = 2 * (i + l) + 2 * n
        let to_signal = |lit: u64| {
            let index = (lit / 2) as usize;
            let inverted = lit % 2 == 1; // only odd literals are inverted
            
            Signal::new(index, inverted)
        };

        // 1. Header lesen (erste Textzeile)
        let mut header_line = String::new();
        reader.read_line(&mut header_line)?;
        let header_parts: Vec<&str> = header_line.trim().split_whitespace().collect();
        if header_parts.len() < 6 || header_parts[0] != "aig" {
            panic!("invalid header!");
        }
        let i: usize = header_parts[2].parse().unwrap();
        let l: usize = header_parts[3].parse().unwrap();
        let o: usize = header_parts[4].parse().unwrap();
        let a: usize = header_parts[5].parse().unwrap();

        // 2. read outputs as ASCII
        let mut outputs = Vec::new();
        for _ in 0..o {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let val_str = line.trim(); // tim(): no whitspace or newlines(\n)
            let val = val_str.parse::<u64>().expect("output not parseable");
            outputs.push(to_signal(val));
        }

        // 3. Read AIG-gates (deltas) and build AIG with own struct
        // Explanation from the paper https://fmv.jku.at/aiger/FORMAT.aiger:
        // "The definition of an AND gate consists of three positive integers all
        // written on one line and separated by exactly one space character.  The
        // first integer is even and represents the literal or left-hand side (LHS).
        // The two other integers represent the literals of the right-hand side
        // (RHS) of the AND gate." With: lhs > rhs0 >= rhs1. This allows to store only the differencs 
        // and save storage. The differences are very small and not negative:
        // delta0 = lhs - rhs0,  delta1 = rhs0 - rhs1. These facts allow the use of little-endian encoding.
        // lhs is not explictly stored because the lhs indices are all consecutive: (I+L+A)
        // input literals = 2, 4, 6, .... 2*i --> input variable indices = literal / 2 or 1, 2, 3, .... i
        // and literal = 2*(I+L)+2, 4*(I+L)+4, .... 2*(I+L+A)   --> and variable indices = i + l + 1  or lhs / 2, da lhs = 2 * (i + l) + 2 * n
        let mut aig = AIG::new(); 
        let base = 2 * (i + l) + 2; //start of the AND indices: (2 * (i + l) + 2)
        let base = base as u64;
        for n in 0..a {
            // read deltas from file
            let delta0 = read_leb(&mut reader)?; 
            let delta1 = read_leb(&mut reader)?;
            
            // calculate the literals (not the indices because the deltas are calculated using the literals)
            let lhs = base + 2 * n as u64;
            let rhs0 = lhs - delta0;
            let rhs1 = rhs0 - delta1;

            // transform literals to signals and create the and-node
            let rhs0_signal = to_signal(rhs0);
            let rhs1_signal = to_signal(rhs1);
            let index = (lhs / 2) as usize;
            aig.create_and(rhs0_signal, rhs1_signal, index);
        }

        // build i inputs
        let mut inputs = Vec::new();
        for k in 0..i {
            let signal = Signal::new((k+1) as usize, false);
            inputs.push(signal);
        }

        Ok(Self { aig, inputs, outputs })
    }

    pub fn aig(&self) -> &AIG {
        &self.aig
    }
    pub fn inputs(&self) -> &Vec<Signal> {
        &self.inputs
    }
    pub fn outputs(&self) -> &Vec<Signal> {
        &self.outputs
    }
}

/// To decode an aiger file, the little endian base decoder is used.
/// Example:
/// 0x85 = 10000101 -> lower 7 bits = 0000101 = 5 (highest bit set -> continue)
/// 0x01 = 00000001 -> lower 7 bits = 0000001 = 1 (highest bit not set -> break)
fn read_leb<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut result = 0u64;
    let mut shift = 0u32; // stores how many bits to shift each 7 bit groupt
    loop {
        let mut buf = [0u8; 1]; // allocate a one byte buffer
        reader.read_exact(&mut buf)?; // read one byte from buffer
        let byte = buf[0];

        // 1. take the 7 lowest bits of the byte. 
        // 2. This works by a bitwise and: 0x7f = 0111 1111. So only the lowest 7 bits are extracted.
        // 3. The byte is casted to an unsigned 64 bit integer
        // 4. The 7 bit chunk is shifted to the correct position inside of the 64 bit integer
        // 5. The now calculated 64 bit integer is now combined with the result variable using the bitwise or.
        result |= ((byte & 0x7F) as u64) << shift; 
        
        // the the highes bit is not set, then stop. Otherwise get the next 7 bits.
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}
