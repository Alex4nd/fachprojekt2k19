extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
// use std::convert::TryFrom;

use crate::traits::WaveletTree;


pub struct PointerWaveletTree<T> {
    alphabet: Vec<T>,
    root: Option<PointerWaveletTreeNode<T>>,
}

enum PointerWaveletTreeNode<T> {
    Node {
        min_element: T,
        max_element: T,
        left_tree: Box<PointerWaveletTreeNode<T>>,
        right_tree: Box<PointerWaveletTreeNode<T>>,
        bits: BitVec<u8>,
    },
    Nil,
}

impl<T: Ord + PartialEq + Clone + Div + Add> PointerWaveletTree<T> {

    pub fn new_fill(data: &[T]) -> PointerWaveletTree<T> {
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
        let size = alphabet.len();
        let mut tree = PointerWaveletTree {
            alphabet: alphabet,
            root: Option::None,
        }; 
        if size > 0 {
            tree.root = Some(PointerWaveletTree::fill_rec(&tree.alphabet[..], data));
        }
        tree
    }

    fn fill_rec(alphabet: &[T], sequence: &[T]) -> PointerWaveletTreeNode<T> {
        if alphabet.len() > 1 {
            
            let mut bits: BitVec<u8> = BitVec::new_fill(false, 32);

            for elem in sequence.iter() {
                let mut position: usize = 0;
                for alph in alphabet.iter() {
                    if elem == alph {
                        if position <= alphabet.len()/2 {
                            bits.set_bit(position as u64, false);
                        }
                        else {
                            bits.set_bit(position as u64, true);
                        }
                        break;
                    }
                    position += 1;
                }
            }

            PointerWaveletTreeNode::Node {
                left_tree: Box::new(PointerWaveletTree::fill_rec(&alphabet[..alphabet.len()/2], &sequence)),
                right_tree: Box::new(PointerWaveletTree::fill_rec(&alphabet[alphabet.len()/2 + 1 ..], &sequence)),
                min_element: Clone::clone(&alphabet[0]),
                max_element: Clone::clone(&alphabet[alphabet.len() - 1]),
                bits, //u64::try_from(data.len()).unwrap());
            }
        } else {
            PointerWaveletTreeNode::Node {
                left_tree: Box::new(PointerWaveletTreeNode::Nil),
                right_tree: Box::new(PointerWaveletTreeNode::Nil),
                min_element: Clone::clone(&alphabet[0]),
                max_element: Clone::clone(&alphabet[0]),
                bits: BitVec::new(),
            } 
        }
    }
}



impl<T> WaveletTree<T> for PointerWaveletTree<T> {

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

    //The position index of the elements in the wavelet tree is assumed to begin at 0
    use super::*;
    use crate::traits::WaveletTree;

    //Tests the function access with valid parameters.
    //The object at the given index 3 does exist in the wavelet-tree, so the expected output is this object
    #[test]
    fn access_success() {
        // let mut data: Vec<String> = Vec::new();
        // data.push(String::from("Albert"));
        // data.push(String::from("Bernd"));
        // data.push(String::from("Connor"));
        // data.push(String::from("Daria"));
        // data.push(String::from("Elena"));

        // let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(&data[..]);
        // let content: String = tree.access(3).unwrap();
        // assert_eq!(content, String::from("Daria"));
    }

    //Tests the function access with an invalid position
    //The object at the given index 5 does not exist in the wavelet-tree, so the expected output is Option::None,
    //to indicate the nonexistence of the object at this position
    #[test]
    fn access_invalid_position() {
        // let mut data: Vec<String> = Vec::new();
        // data.push(String::from("Albert"));
        // data.push(String::from("Bernd"));
        // data.push(String::from("Connor"));
        // data.push(String::from("Daria"));
        // data.push(String::from("Elena"));
        // let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(&data[..]);
        // let content = tree.access(5);
        // assert_eq!(content, Option::None);
    }

    //Tests the function rank with valid parameters
    //The object "1" exists 3 times up to position index 4, so the expected output is 3
    #[test]
    fn rank_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.rank(1, 4);
        assert_eq!(3, content);
    }

    //Tests the function rank with an invalid element
    //The object "42" does not exists in the wavelet tree, so the expected output is 0
    #[test]
    fn rank_invalid_element() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.rank(42, 4);
        assert_eq!(0, content);
    }

    //Tests the function rank with an invalid position index, which is too high
    //The object "1" exists 3 times up to position index 4. Although the index is 5, the expected output is 3
    //An index that exceeds the number of objects in the wavelet tree is tolerated and treated as if it's the highest valid index
    #[test]
    fn rank_position_out_of_bound() {
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

    //Tests the function select with valid parameters
    //The second occurence of the object "0" exists at position index 3 in the wavelet tree, so the expected output is 3
    #[test]
    fn select_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(0, 2);
        assert_eq!(3, content);
    }

    //Tests the function select with an invalid element that does not exist in the wavelet tree
    //The 1st occurence of the object "42" does not exist in the wavelet tree,
    //so the expected output is a panic, to indicate non-existence of the object in the wavleet tree
    #[test]
    #[should_panic]
    fn select_invalid_element() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(42, 1);
        //panic goes here
    }

    //Tests the function select with an invalid occurence that does not exist in the wavelet tree
    //The 4th occurence of the object "1" does not exist in the wavelet tree,
    //so the expected output is a panic, to indicate non-existence of the 4th occurence of the object in the wavleet tree
    #[test]
    #[should_panic]
    fn select_occurence_out_of_bound() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(1, 4);
    }

    //Tests the function select with an invalid occurence of 0
    //The 0th occurence of the object "1" does not make sense,
    //so the expected output is a panic, to indicate the nonsensical nature of this operation
    #[test]
    #[should_panic]
    fn select_occurence_0() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(1, 0);
    }
}


