extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
use num_traits::cast::NumCast;
use num_traits::cast::cast;
// use std::convert::TryFrom;

use std::fmt::Display;
use std::fmt::Debug;

use std::collections::HashMap;
use std::hash::Hash;

use crate::traits::WaveletTree;
use crate::traits::Node;

use crate::pointer::PointerWaveletTree;


pub struct Graph {
    tree: PointerWaveletTree<u64>,
    bits: BitVec<u8>,
    max_node: u64,
}

impl Graph {

    pub fn new_tuple(tuples: &Vec<(u64,u64)>) -> Graph {
        let mut set: HashMap<u64, Vec<u64>> = HashMap::new();
        let mut max = 0; 

        for (a,b) in tuples.iter() {
            if *a > max {
                max = *a;
            }
            if *b > max {
                max = *b;
            }
            if !set.contains_key(a) {
                set.insert(*a, Vec::new());
            }
            if !set.contains_key(b) {
                set.insert(*b, Vec::new());
            }
            let ref mut list = set.get_mut(a).unwrap();
            list.push(*b);
        }
        return Graph::new(&set, max); 
    }

    pub fn new(adjacency: &HashMap<u64, Vec<u64>>, max_node: u64) -> Graph {
        let mut data = Vec::new();
        let mut bits = BitVec::new();

        if adjacency.get(&0).is_some() {
            panic!("Graph shall not contain a node named 0");
        }

        for key in 1..max_node+1 {
            bits.push(true);
            data.push(0);
            let list = adjacency.get(&key);
            if list.is_none() {
                continue;
            }

            for elem in list.unwrap().iter() {
                data.push(*elem);
                bits.push(false);
            }
        }

        let tree = PointerWaveletTree::new_fill(&data);

        Graph {
            tree,
            bits,
            max_node,
        }
    }

    pub fn neighbour(&self, node: u64, i: u64) -> Option<u64> {
        if node > self.max_node {
            return None;
        }

        let rs = RankSelect::new(self.bits.clone(), 1);
        let k = rs.select_1(node).unwrap();

        println!("RS select {}", k);

        return self.tree.access((k + i) as u32);
    }

    pub fn rev_neighbour(&self, node: u64, i: u64) -> Option<u64> {
        if node > self.max_node {
            return None;
        }

        let k = self.tree.select(node, i as u32);

        let rs = RankSelect::new(self.bits.clone(), 1);
        return rs.rank_1(k as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pointer::*;
    use crate::traits::WaveletTree;

    #[test]
    fn constructor_wavelet_tree_correct() {

        let data = vec!((1,2), (2,3), (3,4));

        let mut g = Graph::new_tuple(&data);

        let mut bits = vec!(0,2,0,3,0,4,0);
        assert_eq!(g.tree.to_vec(), Some(bits));
    }

    #[test]
    fn constructor_maxnode_correct() {

        let data = vec!((1,2), (2,3), (3,4));

        let mut g = Graph::new_tuple(&data);

        assert_eq!(g.max_node, 4)
    }
    
    #[test]
    fn constructor_bitvector_correct() {

        let data = vec!((1,2), (2,3), (3,4));

        let mut g = Graph::new_tuple(&data);

        let mut pattern = vec!(1,0,1,0,1,0,1);
        assert_bit_pattern(&mut g.bits, &pattern);
    }

    fn assert_bit_pattern(bits: &mut BitVec<u8>, pattern: &Vec<u32>) {
        println!("Bitvec: {:?}", bits);
        let mut i = 0;
        assert_eq!(bits.len() as usize, pattern.len());
        for bit in pattern.iter() {
            assert_eq!(bits.get(i), *bit == 1, "Bit {} should be {}", i, *bit);
            i += 1;
        }
    }
      
    #[test]
    fn neighbour_single_successor() {

        let data = vec!((1,2), (2,3), (3,4));

        let mut g = Graph::new_tuple(&data);

        assert_eq!(g.neighbour(1, 1), Some(2), "1st neighbour of node 1");
        assert_eq!(g.neighbour(2, 1), Some(3), "1st neighbour of node 2");
        assert_eq!(g.neighbour(3, 1), Some(4), "1st neighbour of node 3");
    }  
    
    #[test]
    fn neighbour_cycle() {

        let data = vec!((1,2), (2,3), (3,4), (4,1), (1,5), (1,6));

        let mut g = Graph::new_tuple(&data);

        assert_eq!(g.neighbour(1, 1),Some(2), "1st neighbour of node 1");
        assert_eq!(g.neighbour(1, 2),Some(5), "2nd neighbour of node 1");
        assert_eq!(g.neighbour(1, 3),Some(6), "3rd neighbour of node 1");
        assert_eq!(g.neighbour(2, 1),Some(3), "1st neighbour of node 2");
        assert_eq!(g.neighbour(3, 1),Some(4), "1st neighbour of node 3");
        assert_eq!(g.neighbour(4, 1),Some(1), "1st neighbour of node 4");
    } 
    
    #[test]
    fn rev_neighbour_cycle() {

        let data = vec!((1,2), (2,3), (3,4), (4,1), (1,5), (1,6));

        let mut g = Graph::new_tuple(&data);

        assert_eq!(g.rev_neighbour(2, 1), Some(1), "1st reverse neighbour of node 2");
        assert_eq!(g.rev_neighbour(3, 1), Some(2), "1st reverse neighbour of node 3");
        assert_eq!(g.rev_neighbour(4, 1), Some(3), "1st reverse neighbour of node 4");
        assert_eq!(g.rev_neighbour(1, 1), Some(4), "1st reverse neighbour of node 1");
        assert_eq!(g.rev_neighbour(5, 1), Some(1), "1st reverse neighbour of node 5");
        assert_eq!(g.rev_neighbour(6, 1), Some(1), "1st reverse neighbour of node 6");
    }  
 
}
