#!/bin/bash

# Eingabeverzeichnis mit AIG-Dateien
AIG_DIR="debug/test_files/aigs"
# Ausgabeziel
OUT_DIR="/Users/jonas/Uni-MA/LS_PD/aigverse/my_cuts_test_files_single_nodes"

# Iteriere über alle aigverse_{i}.aig-Dateien
for aig_file in "$AIG_DIR"/aigverse_*.aig; do
    filename=$(basename "$aig_file")              # z.B. aigverse_2.aig
    base="${filename%.*}"                         # z.B. aigverse_2

    for k in {2..4}; do
        for i in {1..77}; do
            out_file="${OUT_DIR}/${base}_k${k}_node_${i}.txt"

            echo "Verarbeite $filename mit k=$k → $out_file"
            ./target/release/crust -r "$aig_file" -o "$out_file" -k "$k" -c "$i"
        done
    done
done
