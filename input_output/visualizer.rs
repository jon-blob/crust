use crate::aig_structure::aig::AIG;
use crate::aig_structure::signal::Signal;
use std::fs::File;
use std::io::{Result, Write};
use std::process::Command;


pub struct AigVisualizer<'a> {
    pub aig: &'a AIG,
    pub base_path: String
}

impl<'a> AigVisualizer<'a> {
    pub fn new(aig: &'a AIG, base_path: &String) -> Self {
        AigVisualizer { 
            aig,
            base_path: base_path.to_string()
        }
    }

    pub fn export_png(&self, filename: &str, inputs: &[Signal], outputs: &[Signal]) -> Result<()> {
        
        let _ = self.export_dot(filename, inputs, outputs);

        let dotfile = format!("{}/{}.dot", self.base_path, filename);
        let pngfile = format!("{}/{}.png", self.base_path, filename);

        let status = Command::new("dot")
        .args(&["-Tpng", &dotfile, "-o", &pngfile])
        .status()
        .expect("Failed to execute dot command");

        if status.success() {
            println!("Image succesfully created!");
        } else {
            eprintln!("dot-error: {}", status);
        }
        
        Ok(())

    }

    pub fn export_dot(&self, filename: &str, inputs: &[Signal], outputs: &[Signal]) -> Result<()> {
        let path = format!("{}/{}.dot", self.base_path, filename);
        let mut file = File::create(&path)?;
        writeln!(file, "digraph AIG {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        // Inputs
        for input in inputs {
            writeln!(
                file,
                "  x{} [label=\"x{}\", shape=box, style=filled, fillcolor=lightblue];",
                input.index, input.index
            )?;
        }

        // AND-Nodes
        for (index, node) in &self.aig.node_map {
            writeln!(file, "  x{} [label=\"x{}\"];", index, index)?;

            for input in &[node.left_signal, node.right_signal] {
                let style = if input.inverted { "dashed" } else { "solid" };
                writeln!(
                    file,
                    "  x{} -> x{} [style={}];",
                    input.index, index, style
                )?;
            }
        }

        // Outputs
        for (i, output) in outputs.iter().enumerate() {
            let style = if output.inverted { "dashed" } else { "solid" };
            let label = if output.inverted {
                format!("f{} = ¬x{}", i, output.index)
            } else {
                format!("f{} = x{}", i, output.index)
            };

            writeln!(
                file,
                "  f{} [label=\"{}\", shape=diamond, style=filled, fillcolor=lightgreen];",
                i, label
            )?;
            writeln!(file, "  x{} -> f{} [style={}];", output.index, i, style)?;
        }

        writeln!(file, "}}")?;




        Ok(())
    }
}
