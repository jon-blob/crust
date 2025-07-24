#![allow(warnings)]
use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Write};

mod aig_structure;
mod algorithms;
mod input_output;

use aig_structure::aig::AIG;
use crate::algorithms::cut_enumerator::CutEnumerator;
use crate::input_output::read_aiger::AigerReader;
use crate::input_output::visualizer::AigVisualizer;
use std::path::Path;


/// AIG Processing Tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// AIGER input file
    #[arg(short = 'r', long)]
    read_aiger: String,

    /// calculate cuts for all nodes
    #[arg(short = 'e', long)]
    cut_enumerate: Option<String>,

    /// calculate cuts for a single node
    #[arg(short = 'c', long)]
    cut: Option<usize>,

    /// Enable graph visualization
    #[arg(short = 'v', long)]
    visualize: Option<String>,

    /// Maximum cut size (optional, default = 4)
    #[arg(short = 'k', long, default_value_t = 4)]
    max_cut_size: usize,

    /// Optional output path for single node cut result
    #[arg(short = 'o', long)]
    cut_output: Option<String>,

}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut reader = AigerReader::from_file(&args.read_aiger)?;
    let aig = &mut reader.aig;

    if let Some(path) = &args.visualize {
        let full_path = Path::new(path);
        // 1. part: output_path (as &str)
        let output_path = full_path.parent().unwrap().to_str().unwrap();
         // 2. part: filename without .png
        let file_stem = full_path.file_stem().unwrap().to_str().unwrap();

        let exporter = AigVisualizer::new(aig, &output_path.to_string());
        exporter.export_png(&file_stem, &reader.inputs, &reader.outputs)?;
        println!("Graph visualized at {output_path}/{file_stem}.png");
    }

    if let Some(path) = args.cut_enumerate {
        let mut cut_enumerator = CutEnumerator::new(aig);
        cut_enumerator.enumerate_cuts(args.max_cut_size, &reader.inputs);

        fs::create_dir_all(
            std::path::Path::new(&path).parent().unwrap_or_else(|| ".".as_ref())
        )?;
        let mut file = File::create(&path)?;
        writeln!(file, "{:?}", cut_enumerator.cuts)?;
        println!("Cuts written to {path}");
        
    }

    if let Some(target_node) = args.cut {
        let mut cut_enumerator = CutEnumerator::new(aig);
        let cuts_for_target_node = cut_enumerator.calculate_cuts_single_node(
            args.max_cut_size,
            &reader.inputs,
            target_node,
        );

        if let Some(output_path) = &args.cut_output {
            fs::create_dir_all(
                std::path::Path::new(&output_path).parent().unwrap_or_else(|| ".".as_ref())
            )?;
            let mut file = File::create(output_path)?;
            writeln!(file, "{:?}", cuts_for_target_node)?;
            println!("Cuts for node {target_node} written to {output_path}");
        } else {
            println!("Cuts for node {target_node}: {:?}", cuts_for_target_node);
        }
    }

    Ok(())
}
