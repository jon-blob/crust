use crate::aig_structure::signal::Signal;
use crate::aig_structure::and_node::AndNode;
use std::collections::{HashMap, HashSet};

/// Define an AIG struct.
/// compute_table: used to check whether an AndNode has been calculated or not
/// node_map: HashMap that contains all AndNodes and the id.
pub struct AIG {
    pub compute_table: HashMap<(Signal, Signal), Signal>,
    pub node_map: HashMap<usize, AndNode>,
}
impl AIG {
    pub fn new() -> Self {
        AIG {
            compute_table: HashMap::new(),
            node_map: HashMap::new()
        }
    }

    pub fn create_and(&mut self, mut a: Signal, mut b: Signal, new_index: usize) -> Signal {
        (a, b) = Self::check_swap(a, b);

        (a, b) = Self::check_swap(a, b);                        // swap if a > b

        if a.index == 0 && !a.inverted {                        // 0 and b = 0 remember: the signal (0, false) or (x_0, 0) represents the constant 0.  
            return Signal::new(0, false);
        }

        if a.index == 0 && a.inverted{                          // 1 and b = b remember: the signal (0, true) or (x_0, 1) represents the constant 1.
            return b;
        }

        if a.index == b.index && a.inverted != b.inverted{      // a and !a = 0
            return Signal::new(0, false);
        }

        if a.index == b.index && a.inverted == b.inverted{      // a and a = a
            return a;
        }
        
        if let Some(&result) = self.compute_table.get(&(a, b)) { // check if x_i in compute table then return (x_i, 0)
            return Signal::new(result.index, false);
        } 

        // create a new AndNode an add it to the node_map
        let new_signal = Signal::new(new_index, false);
        self.compute_table.insert((a, b), new_signal);
        self.node_map.insert(new_signal.index, AndNode{left_signal: a, right_signal: b});

        return new_signal;
    }

    /// swap if a > b
    fn check_swap(a: Signal, b: Signal) -> (Signal, Signal) {
        if a.index > b.index { (b, a) } else { (a, b) }
    }

    /// Preparation for the topologically traverse part of the cut enumeration algorithm. 
    /// We do a topolical sort. That means we create a list of signals, so that for every directed edge (a,b),
    /// a comes before b. This means if we do the cut enumeration algorithm we can process each node after all its 
    /// predecessors have been processed. If we want to process the node 5 in the example below, we have to process 
    /// the node 4 and the input x3. The order is in this case [1,2,4,3,5]
    /// 
    /// visited: contains the visited nodes in a hashset to make sure, we don't visit a node twice.
    /// order: the topoligally sorted order of nodes
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        
        // the for-loop is necessary, to make sure that we visit every node. But why not start with the biggest node id? 
        // If we always begin with the biggest node we might not visit all nodes. Lecture 6 (aig_optimization ) Slide 11 
        // shows an example where it is necessary. If we start with node 9 we don't visit node 8 with this algorithm.
        for &node_id in self.node_map.keys() {  
            self.topological_visit(node_id, &mut visited, &mut order);
        }

        order
    }

    /// Classic recursive search through the tree. 
    /// Example:            x1      x2      x3
    ///                      \      /       /
    ///                         4          /
    ///                          \        /
    ///                           \      /
    ///                               5  
    ///                               | 
    /// If the node map is [5,4] then we start with the search at 5. First we go to the left signal and insert it into our order vector.
    /// Then we go again to the left signal and we insert the id into the order vector. Because x1 is no node the if clause is not valid.
    /// Then we go back to the 4 node and go to the right signal x2. And so on....
    fn topological_visit(&self, node_id: usize, visited: &mut HashSet<usize>, order: &mut Vec<usize>) {
        if visited.contains(&node_id) {
            return;
        }

        visited.insert(node_id);

        if let Some(node) = self.node_map.get(&node_id) {
            self.topological_visit(node.left_signal.index, visited, order);
            self.topological_visit(node.right_signal.index, visited, order);
        }

        order.push(node_id);
    }


}
