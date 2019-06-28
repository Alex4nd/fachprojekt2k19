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


struct Graph {
    tree: PointerWaveletTree<u64>,
    bits: BitVec<u8>,
}

impl Graph {

    pub fn new<T>(adjacency: &Vec<(T,T)>) -> Graph 
        where T: Eq + Hash {
        
        let mut set: HashMap<&T, (u64,Vec<&T>)> = HashMap::new();
        let mut index = 0;
        for (a,b) in adjacency.iter() {
            if !set.contains_key(a) {
                set.insert(a, (index, Vec::new()));
                index += 1;
            }
            let (idx, ref mut list) = set.get_mut(a).unwrap();
            list.push(b);
        }

        let mut data = Vec::new();

        for (key, (idx,list)) in set.iter() {
            for elem in list.iter() {
                let (idx, l) = set.get(elem).unwrap();
                data.push(*idx);
            }
        }

        let tree = PointerWaveletTree::new_fill(&data);
        let bits = BitVec::new();

        Graph {
            tree,
            bits,
        }
    }

}
