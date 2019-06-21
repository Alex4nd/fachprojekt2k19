extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
use num_traits::cast::NumCast;
use num_traits::cast::cast;
// use std::convert::TryFrom;

use crate::traits::WaveletTree;
use crate::traits::Node;

pub struct PointerWaveletTree<T> {
    alphabet: Vec<T>,
    root: Option<PointerWaveletTreeNode<T>>,
}

#[derive(Debug, PartialEq)]
struct PointerWaveletTreeNode<T> {
        min_element: T,
        max_element: T,
        left_tree: Option<Box<PointerWaveletTreeNode<T>>>,
        right_tree: Option<Box<PointerWaveletTreeNode<T>>>,
        bits: BitVec<u8>,
}

impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> PointerWaveletTree<T> {

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

            PointerWaveletTreeNode{
                left_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[..alphabet.len()/2], &sequence))),
                right_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[alphabet.len()/2 + 1 ..], &sequence))),
                min_element: Clone::clone(&alphabet[0]),
                max_element: Clone::clone(&alphabet[alphabet.len() - 1]),
                bits, //u64::try_from(data.len()).unwrap());
            }
        } else {
            PointerWaveletTreeNode{
                left_tree: Option::None,
                right_tree: Option::None,
                min_element: Clone::clone(&alphabet[0]),
                max_element: Clone::clone(&alphabet[0]),
                bits: BitVec::new(),
            }
        }
    }
}

impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> Node<T> for PointerWaveletTreeNode<T> {

    fn isLeaf(&self) -> bool{
        match &self.left_tree{
            Some(_) => false,
            
            None => {
                match &self.right_tree {
                    Some(_) => false,
                    
                    None => true,
                
                }
            },
        }
    }
    
    
    fn access(&self, index: u32) -> T{
        let currentnode = &self;
        while currentnode.max_element != currentnode.min_element {
            let rs = RankSelect::new(self.bits.clone(),1);
            if self.bits[index as u64] == true{
                let result = rs.rank_1(index as u64);
                let currentnode = &currentnode.right_tree;
            }
            else{
                let result = rs.rank_0(index as u64);
                let currentnode = &currentnode.left_tree;
            }
        }
        currentnode.min_element
    }

    fn rank(&self, element: T, index: u32) -> u32 {
        if &self.min_element == &self.max_element{
            index
        }
        else{
            let two: T = cast(2).unwrap();
            let rs = RankSelect::new(self.bits.clone(),1);
            if  element < (self.min_element + self.max_element) / two{
                let result = rs.rank_0(index as u64);
                match result{
                
                    Some(ind) => {
                        match &
                        self.left_tree {
                            Some(node) => node.rank(element, ind as u32),
                            
                            None => panic!("Der Baum sollte hier einen Knoten haben, hat aber keinen"),
                        }
                    },
                    
                    None => panic!("Invalider Indexwert für binäres Rank"),
                }
            }
            else{
                let result = rs.rank_1(index as u64);
                match result{
                
                    Some(ind) => {
                        match &self.right_tree {
                            Some(node) => node.rank(element, ind as u32),
                            
                            None => panic!("Der Baum sollte hier einen Knoten haben, hat aber keinen"),
                        }
                    },
                    
                    
                    None => panic!("Invalider Indexwert für binäres Rank"),
                }
            }
        }
    }


    fn select(&self, element: T, index: u32) -> u32{
        if self.isLeaf(){
            index
        }
        else{
            let two: T = cast(2).unwrap();
            if element < (self.min_element + self.max_element) / two{
                let sel = match &self.left_tree {
                    Some(node) => node.select(element, index),
                    
                    None => panic!("Der Baum sollte hier einen Knoten haben, hat aber keinen"),
                    };
                let rs = RankSelect::new(self.bits.clone(),1);
                let result = rs.select_0(sel as u64);
                match result{
                
                    Some(ind) => ind as u32,
                    
                    None => panic!("Invalider Indexwert für binäres Select"),
                }
            }
            else{
                let sel = match &self.right_tree {
                    Some(node) => node.select(element, index),
                    
                    None => panic!("Der Baum sollte hier einen Knoten haben, hat aber keinen"),
                };
                let rs = RankSelect::new(self.bits.clone(),1);
                let result = rs.select_1(sel as u64);
                match result{
                
                    Some(ind) => ind as u32,
                    
                    None => panic!("Invalider Indexwert für binäres Select"),
                }
            }
        }
    }
    
    
}

impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> WaveletTree<T> for PointerWaveletTree<T> {

    fn access(&self, index: u32) -> std::option::Option<T>{
        let root = match &self.root{
            Some(node) => node,
            
            None => panic!("Kein Wavelettree vorhanden"),
        };
        if index > root.bits.len() as u32 {
            None
        }
        else{
            Some(root.access(index))
        }
    }
    
    fn rank(&self, element: T, index: u32) -> u32 {
        let root = &self.root;
        match root{
            Some(root) => root.rank(element, index),
            
            None => panic!("Kein Wavelettree vorhanden"),
        }
    }

    fn select(&self, element: T, index: u32) -> u32{
        let root = &self.root;
        match root{
            Some(root) => root.select(element, index),
            
            None => panic!("Kein Wavelettree vorhanden"),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::WaveletTree;

    //Tests the compatibility with the primitive T = u8 as a char representation
    #[test]
    fn char_compatibility(){
	let mut data: Vec<u8> = Vec::new();
        data.push(b'a');
        data.push(b'b');
        data.push(b'c');
        data.push(b'd');
        data.push(b'e');
        let tree: PointerWaveletTree<u8> = PointerWaveletTree::new_fill(&data[..]);
        let content = tree.access(3).unwrap();
        assert_eq!(content, b'd');
    }

    //Tests if the creation with empty data is functional, assuming the function is used to generate empty tree nodes
    #[test]
    fn creation_empty_data(){
	let mut data: Vec<u32> = Vec::new();
	let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
	let empty_node = Option::None;
	assert_eq!(tree.root, empty_node);
    }

    //Tests if the creation with non-empty data is functional
    //specifically, if the bit vector is initialized correctly
    #[test]
    fn creation_non_empty_data(){
	let mut data: Vec<u32> = Vec::new();
        data.push(4);
        data.push(2);
        data.push(4);
        data.push(2);
        data.push(4);
	let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
	let mut test_bits = BitVec::new_fill(false, 32);
	test_bits.set_bit(0 as u64, true);
	test_bits.set_bit(2 as u64, true);
	test_bits.set_bit(4 as u64, true);
	assert_eq!(tree.root.unwrap().bits, test_bits);
    }

    //The position index of the elements in the wavelet tree is assumed to begin at 0

    //Tests the function access with valid parameters.
    //The object at the given index 3 does exist in the wavelet-tree, so the expected output is this object
    #[test]
    fn access_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content = tree.access(3).unwrap();
        assert_eq!(content, 0);
    }

    //Tests the function access with an invalid position
    //The object at the given index 5 does not exist in the wavelet-tree, so the expected output is Option::None,
    //to indicate the nonexistence of the object at this position
    #[test]
    fn access_invalid_position() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content = tree.access(5);
        assert_eq!(content, Option::None);
    }

    //Tests the function rank with valid parameters
    //The object "1" exists 3 times up to position index 4, so the expected output is 3
    #[test]
    fn access() {
         //let tree: PointerWaveletTree<u32> = PointerWaveletTree::new(64);
         //tree.access(5);
    }
    
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
    fn rank() {
    }
    
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
    fn select() {
    }
    
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


