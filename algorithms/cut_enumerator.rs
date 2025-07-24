#![allow(warnings)]

use std::collections::{HashMap, HashSet};
use crate::aig_structure::aig::AIG;
use crate::aig_structure::signal::Signal;

/// CutEnumerator calculates all k-feasible cuts
/// aig: graph for which the cuts are calculated
/// cuts: HashMap containing all cuts
/// topo_order: vector cotaining all nodes in a topological order
/// num_inputs: number of input signals
pub struct CutEnumerator<'a> {
    pub aig: &'a AIG,
    pub cuts: HashMap<usize, Vec<HashSet<usize>>>,
    pub topo_order: Vec<usize>,
    pub num_inputs: usize,
}

impl<'a> CutEnumerator<'a> {
    pub fn new(aig: &'a AIG) -> Self {
        CutEnumerator {
            aig,
            cuts: HashMap::new(),
            topo_order: Vec::new(),
            num_inputs: 0,
        }
    }

    /// Calculates all minimal cuts for a single nodes
    /// cut_size: maximum number of leaves for a cut.
    /// inputs: list of all input signals
    /// target_node: calculate all k-feasible cuts for this node
    // TODO: cut_limit: maximum number of cuts for a node.
    pub fn calculate_cuts_single_node(&mut self, cut_size: usize, inputs: &[Signal], target_node: usize) -> Vec<HashSet<usize>> {
        self.cuts.clear();
        self.topo_order = self.aig.topological_sort();

        // check if the target node is part of the AIG. If not return an empty vector.
        let is_in_input = inputs.iter().any(|sig| sig.index == target_node);
        let is_in_topo = self.topo_order.contains(&target_node);
        if !is_in_input && !is_in_topo {
            eprintln!("Warning: target_node {target_node} not found in AIG or inputs.");
            return vec![];
        }

        // get all relevant nodes to compute the cuts of the target_node
        let mut relevant_nodes = Vec::new();
        if let Some(pos) = self.topo_order.iter().position(|&id| id == target_node) {
            relevant_nodes = self.topo_order[..=pos].to_vec(); // inclusive slice
        }

        // If there are no AndNodes in the graph, then topo order would be empty.
        // But what if we have just a constant as an output? Then we only need to compute
        // cuts for the target node itself.
        if relevant_nodes.is_empty() {
            relevant_nodes = vec![target_node];
        }

        // 1. topologically traverse
        for &node_idx in relevant_nodes.iter() {
            if let Some(node) = self.aig.node_map.get(&node_idx) {
                // it is an AndNode
                let new_cuts = self.compute_node_cuts(node_idx, cut_size);
                let mut minimal = Self::filter_minimal_cuts(&new_cuts);
                
                // the node is always part of its own cut and is added at the end. 
                // This is necessary for the cut_limit implementation.
                let mut set = HashSet::new();
                set.insert(node_idx);
                minimal.push(set);

                self.cuts.insert(node_idx, minimal);
            } else {
                // it is not an AndNode -> Input
                let mut set = HashSet::new();
                set.insert(node_idx);
                self.cuts.insert(node_idx, vec![set]);
            }
        }

        self.cuts.get(&target_node).cloned().unwrap_or_default()

    }

    /// Calculates all minimal cuts for all nodes
    /// cut_size: maximum number of leaves for a cut.
    /// inputs: list of all input signals
    // TODO: cut_limit: maximum number of cuts for a node.
    pub fn enumerate_cuts(&mut self, cut_size: usize, inputs: &[Signal]) {
        self.cuts.clear();
        self.topo_order = self.aig.topological_sort();

        // If there are no AndNodes in the graph, then topo order would be empty.
        // But what if we have just a constant as an output? Therefore we have to fill
        // the topo order with the input ids.
        if self.topo_order.is_empty() {
            self.topo_order = inputs.iter().map(|sig| sig.index).collect();
        }

        // 1. topologically traverse
        for &node_idx in self.topo_order.iter() {
            if let Some(node) = self.aig.node_map.get(&node_idx) {
                // it is an AndNode
                let new_cuts = self.compute_node_cuts(node_idx, cut_size);
                let mut minimal = Self::filter_minimal_cuts(&new_cuts);
                
                // the node is always part of its own cut and is added at the end. 
                // This is necessary for the cut_limit implementation.
                let mut set = HashSet::new();
                set.insert(node_idx);
                minimal.push(set);

                self.cuts.insert(node_idx, minimal);
            } else {
                // it is not an AndNode -> Input
                let mut set = HashSet::new();
                set.insert(node_idx);
                self.cuts.insert(node_idx, vec![set]);
            }
        }
    }

    /// compute all non-filtered cuts for a given node
    fn compute_node_cuts(&self, node_idx: usize, cut_size: usize) -> Vec<HashSet<usize>> {
        let node = self.aig.node_map.get(&node_idx).unwrap();
        
        // get both fanins
        let left = node.left_signal.index;
        let right = node.right_signal.index;
        
        let mut new_cuts = Vec::new();
        
        // for each cut_l and for each cut_r union = cut_l ∪ cut_r
        for cut_l in &self.cuts[&left] {
            for cut_r in &self.cuts[&right] {
                let union: HashSet<_> = cut_l.union(cut_r).cloned().collect();

                // add union only to new_cuts if |union| <= cut_size
                if union.len() <= cut_size {
                    new_cuts.push(union);
                }
            }
        }

        new_cuts
    }

    /// filter the cuts. No cut is dominated by another cut
    /// c_1 dominates c_2 if c_1 ⊂ c_2 (we take c_1)
    fn filter_minimal_cuts(all_cuts: &[HashSet<usize>]) -> Vec<HashSet<usize>> {
    let mut result = Vec::new();

    'outer: for (i, c_2) in all_cuts.iter().enumerate() {
        // check if 
        for (j, c_1) in all_cuts.iter().enumerate() {
            if i != j && c_1.is_subset(c_2) && c_1 != c_2 {
                // c_2 wird von c_1 dominiert -> nicht minimal
                continue 'outer;
            }
        }
        // Prüfe auf Duplikate in result
        if !result.iter().any(|x| x == c_2) {
            result.push(c_2.clone());
        }
    }
    result
}

}