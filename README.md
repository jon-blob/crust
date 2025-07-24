# crust

**crust** is a Rust-based package that implements the **cut enumeration algorithm** for *And-Inverter Graphs (AIGs)*.  
It can read AIGs from **AIGER files**, generate **PNG visualizations**, and enumerate **all k-feasible cuts**.

## Requirements

To use `crust`, you need to have **Rust** installed:  
👉 [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

If you want to visualize the graph you also need to install `graphviz`:  
👉 [https://graphviz.org/download/](https://graphviz.org/download/)

## Installation & Build

If you make any changes to the code or want to build `crust` yourself, run the following command from within the `crust` directory:

```bash
cargo build --release
```

The resulting binary will be located at `./target/release/crust`.

## Usage

Example command:

```bash
./target/release/crust \
  -r debug/test_files/aigs/aigverse_0.aig \
  -v debug/test_files/crust/aigverse_0.png \
  -e debug/test_files/crust/aigverse_0.txt \
  -c 3 \
  -k 3 \
  -o debug/test_files/crust/aigverse_0_cuts_node_3.txt
```

### Option Descriptions

- `-r <path_to_aig>`  
  Loads an AIG from an AIGER file.

- `-v <path_to_png>`  
  Generates a PNG visualization of the AIG.

- `-e <path_to_store_cuts>`  
  Writes all computed cuts for all nodes to a text file. (Default: `cut_size = 4`)

- `-c <integer>`
  Prints all computed cuts for a single node. (Default: `cut_size = 4`)

- `-k <integer>`  
  Specifies the maximum cut size (`k`).

- `-o <path_to_store_cuts>`
  Writes all computed cuts for a single nodes to a text file.

⚠️ Always start the command with `./target/release/crust` to run the program.

## Project Structure

```text
crust/
├── aig_structure/       # Code for AIG representation and management
├── algorithms/          # Implemented algorithms (currently: cut enumeration)
├── debug/               # Example AIGER files (created with aigverse) & images for debugging/testing
├── input_output/        # AIGER file import & PNG export
├── target/              # Auto-generated Rust build files
└── main.rs              # Entry point with command-line interface
```
