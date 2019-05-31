extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

// use crate::traits;
use crate::traits::WaveletTree;

// use std::convert::TryFrom;

pub struct PointerWaveletTree<'a, T> {
    alphabet: Vec<T>,
    root: Option<PointerWaveletTreeNode<'a, T>>,
}

enum PointerWaveletTreeNode<'a, T> {
    Node {
        minElement: &'a T,
        maxElement: &'a T,
        leftTree: Box<PointerWaveletTreeNode<'a, T>>,
        rightTree: Box<PointerWaveletTreeNode<'a, T>>,
        bits: BitVec<u8>,
    },
    Nil,
}

impl<'a, T: Ord + PartialEq + Clone> PointerWaveletTree<'a, T> {

    pub fn new_fill(data: &[T]) -> PointerWaveletTree<'a, T> {
        let mut alphabet: Vec<T> = Vec::new();
        for elem in data.iter() {
            let mut found = false; 
            for alph in alphabet.iter() {
                if elem == alph {
                    found = true;
                    break;
                }
            } 
            if !found {
                alphabet.push(Clone::clone(elem));
            }
        }
        alphabet.sort();
        PointerWaveletTree {
            alphabet: alphabet,
            root: Option::None,
        } 
    }

    fn fill_rec(alphabet: &'a [T], sequence: &[T]) -> PointerWaveletTreeNode<'a, T> {
        if alphabet.len() > 1 {
            PointerWaveletTreeNode::Node {
                leftTree: Box::new(PointerWaveletTree::fill_rec(&alphabet[..alphabet.len()/2], &sequence)),
                rightTree: Box::new(PointerWaveletTree::fill_rec(&alphabet[alphabet.len()/2 + 1 ..],&sequence)),
                minElement: &alphabet[0],
                maxElement: &alphabet[alphabet.len() - 1],
                bits: BitVec::new_fill(false, 32), //u64::try_from(data.len()).unwrap());
            }
        } else {
            PointerWaveletTreeNode::Nil 
        }
    }
}



impl<'a, T> WaveletTree<T> for PointerWaveletTree<'a, T> {

    fn access(&self, index: u32) {
    
    }

    fn rank(&self, element: T, index: u32) {
    
    }

    fn select(&self, element: T, index: u32) {
    
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::traits::WaveletTree;

    #[test]
    fn access() {
        let x = vec![64,32,4];
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&x[..]);
        tree.access(5)
    }

    #[test]
    fn rank() {
    
    }

    #[test]
    fn select() {
    
    }
}


