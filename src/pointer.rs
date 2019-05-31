extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
// use std::convert::TryFrom;

use crate::traits::WaveletTree;


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

impl<'a, T: Ord + PartialEq + Clone + Div + Add> PointerWaveletTree<'a, T> {

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
        let tree = PointerWaveletTree {
            alphabet: alphabet,
            root: Option::None,
        }; 
        PointerWaveletTree::fill_rec(&tree.alphabet[..], data);
        tree
    }

    fn fill_rec(alphabet: &'a [T], sequence: &[T]) -> PointerWaveletTreeNode<'a, T> {
        if alphabet.len() > 1 {
            
            let mut bits: BitVec<u8> = BitVec::new_fill(false, 32);

            let mut length: usize = 0;
            for elem in sequence.iter() {
                let mut position: usize = 0;
                for alph in alphabet.iter() {
                    position += 1;
                    if elem == alph {
                        length += 1;
                        if position <= alphabet.len()/2 {
                            bits.set_bit(position as u64, false);
                        }
                        else {
                            bits.set_bit(position as u64, true);
                        }
                        break;
                    }
                }
            }

            PointerWaveletTreeNode::Node {
                leftTree: Box::new(PointerWaveletTree::fill_rec(&alphabet[..alphabet.len()/2], &sequence)),
                rightTree: Box::new(PointerWaveletTree::fill_rec(&alphabet[alphabet.len()/2 + 1 ..], &sequence)),
                minElement: &alphabet[0],
                maxElement: &alphabet[alphabet.len() - 1],
                bits, //u64::try_from(data.len()).unwrap());
            }
        } else {
            PointerWaveletTreeNode::Node {
                leftTree: Box::new(PointerWaveletTreeNode::Nil),
                rightTree: Box::new(PointerWaveletTreeNode::Nil),
                minElement: &alphabet[0],
                maxElement: &alphabet[0],
                bits: BitVec::new(),
            } 
        }
    }
}



impl<'a, T> WaveletTree<T> for PointerWaveletTree<'a, T> {

    fn access(&self, index: u32) -> Option<T>{
    
	return Option::None;
    }

    fn rank(&self, element: T, index: u32) -> u32{
    
	return 42;
    }

    fn select(&self, element: T, index: u32) -> u32{
    
	return 42;
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::traits::WaveletTree;

    // #[test]
    // fn access() {
	// let mut data: Vec<String> = Vec::new();
	// data.push(String::from("Albert"));
	// data.push(String::from("Bernd"));
	// data.push(String::from("Connor"));
	// data.push(String::from("Daria"));
	// data.push(String::from("Elena"));
    //     let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(&data[..]);
    //     let content: String = tree.access(4).unwrap();
	// assert_eq!(content, String::from("Daria"));
    // }

    #[test]
    fn rank() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.rank(1, 5);
	assert_eq!(3, content);
    }

    #[test]
    fn select() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(0, 2);
	assert_eq!(4, content);
    }
}


