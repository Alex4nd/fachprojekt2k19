extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;


// use crate::traits;
use crate::traits::WaveletTree;

// use std::convert::TryFrom;

enum PointerWaveletTree<T> {
    Node {
        leftAlphabet: Vec<T>,
        rightAlphabet: Vec<T>,
        leftTree: Box<PointerWaveletTree<T>>,
        rightTree: Box<PointerWaveletTree<T>>,
        bits: BitVec<u8>,
    },
    Nil,
}

impl<T: Ord + PartialEq + Clone> PointerWaveletTree<T> {

    fn new(capacity: u64) -> PointerWaveletTree<T> {
        PointerWaveletTree::Node {
            leftAlphabet: Vec::new(),
            rightAlphabet: Vec::new(),
            leftTree: Box::new(PointerWaveletTree::Nil),
            rightTree: Box::new(PointerWaveletTree::Nil),
            bits: BitVec::new_fill(false, capacity)
        }
    }

    fn new_fill(data: Vec<T>) -> PointerWaveletTree<T> {
        let mut alphabet: Vec<T> = Vec::new();
        for elem in data.iter() {
            let mut found = false; 
            for alph in alphabet.iter() {
                if (elem == alph) {
                    found = true;
                    break;
                }
            } 
            if (!found) {
                alphabet.push(Clone::clone(elem));
            }
        }
        alphabet.sort();
        let rightAlphabet = alphabet.split_off(alphabet.len()/2);
        let leftAlphabet = alphabet;
        PointerWaveletTree::Node {
            leftTree: Box::new(PointerWaveletTree::fill_rec(Clone::clone(&leftAlphabet), &data)),
            rightTree: Box::new(PointerWaveletTree::fill_rec(Clone::clone(&rightAlphabet),&data)),
            rightAlphabet,
            leftAlphabet,
            bits: BitVec::new_fill(false, 32), //u64::try_from(data.len()).unwrap());
        }
    }

    fn fill_rec(alphabet: Vec<T>, sequence: &Vec<T>) -> PointerWaveletTree<T> {
        PointerWaveletTree::Nil
    }
}

impl<T> WaveletTree<T> for PointerWaveletTree<T> {

    fn access(&self, index: u32) -> T{
        let bit = bits[index];
        if (bit == true){
            if(rightAlphabet.len() == 1)
                rightAlphabet[0];
            else
                let rs = RankSelect::new(bits,1);
                let ranked = rs.rank(index);
                rightTree.access(ranked);
                
        }
        else{
            if(leftAlphabet.len() == 1)
                leftAlphabet[0];
            else
                let rs = RankSelect::new(bits,1);
                let ranked = index - rs.rank(index);
                leftTree.access(ranked);
        }
        
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
         let tree: PointerWaveletTree<u32> = PointerWaveletTree::new(64);
         tree.access(5)
    }

    #[test]
    fn rank() {
    
    }

    #[test]
    fn select() {
    
    }
}


