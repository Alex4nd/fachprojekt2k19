extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
// use std::convert::TryFrom;

use std::fmt::Display;
use std::fmt::Debug;

use crate::traits::WaveletTree;


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

impl<T: Ord + PartialEq + Clone + Debug + Display + Div + Add> PointerWaveletTree<T> {

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
        println!("recusrive tree fill with \n\talphabet {:?} (len: {})", alphabet, alphabet.len());
        
        
        if alphabet.len() > 1 {
            let mut bits: BitVec<u8> = BitVec::new();

            let exp = f32::ceil( f32::log2(alphabet.len() as f32) ) as usize;
            let middle = 2i8.pow( exp as u32 - 1 ) as usize;

            let mut length = 0;
            for elem in sequence.iter() {
                let mut position: usize = 0;
                for alph in alphabet.iter() {
                    if elem == alph {
                        println!("Symbol {} at index {} is in alphabet {:?} -> {}", elem, length, alphabet, position >= middle);
                        if position < middle {
                            bits.push(false);
                        }
                        else {
                            bits.push(true);
                        }
                        length += 1;
                        break;
                    }
                    position += 1;
                }
            }
            
            PointerWaveletTreeNode {
                left_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[.. middle], &sequence))),
                right_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[middle ..], &sequence))),
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

    pub fn level_order_bits(&self) -> BitVec<u8> {
        let mut result: BitVec<u8> = BitVec::new();
        let tree = &self.root;
        match tree {
            Option::None => {}
            Option::Some(_) => {
                //for ( i = 0; i < tree.){
                //    result.push(bit);
                //}
            }
        }
        result
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
    use super::*;
    use crate::traits::WaveletTree;

    #[test]
    fn constructor_alphabet() {
        let data = vec!(1,4,1,2,1,5,0,1,0,4,1,0,1,4,1,2,1,5,3,1);
        
        let tree = PointerWaveletTree::new_fill(&data);
   
        assert!(tree.alphabet.len() == 6);
    }

    #[test]
    fn constructor_data() {
        // aiabar a ia aiabarda
        // 14121501041014121531
        let data = vec!(1,4,1,2,1,5,0,1,0,4,1,0,1,4,1,2,1,5,3,1);
        
        let mut tree = PointerWaveletTree::new_fill(&data);
   
        assert!(tree.root.is_some());

        let pattern = vec!(0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,1,0);
		assert_node(tree_traversal(&mut tree, "."), &pattern, &0, &5);

        let pattern = vec!(0,0,1,0,0,0,0,0,0,0,0,1,0,0);
		assert_node(tree_traversal(&mut tree, "l"), &pattern, &0, &2);

        let pattern = vec!(1,1,1,0,1,0,1,0,1,1,1,1);
		assert_node(tree_traversal(&mut tree, "ll"), &pattern, &0, &1);

		assert_leaf(tree_traversal(&mut tree, "lll"), &0);
		assert_leaf(tree_traversal(&mut tree, "llr"), &1);
		assert_leaf(tree_traversal(&mut tree, "lr"), &2);

        let pattern = vec!(0,1,0,0,1,0);
		assert_node(tree_traversal(&mut tree, "r"), &pattern, &3, &5);

        let pattern = vec!(1,1,1,0);
		assert_node(tree_traversal(&mut tree, "rl"), &pattern, &3, &4);

		assert_leaf(tree_traversal(&mut tree, "rll"), &3);
		assert_leaf(tree_traversal(&mut tree, "rlr"), &4);
		assert_leaf(tree_traversal(&mut tree, "rr"), &5);
    }

    fn tree_traversal<'a, T: PartialEq + Debug>(tree: &'a mut PointerWaveletTree<T>, path: &str) -> &'a mut PointerWaveletTreeNode<T> {

		use std::borrow::BorrowMut;

        assert!(path.is_ascii());
		assert!(tree.root.is_some());
        let mut rv: &mut PointerWaveletTreeNode<T> = tree.root.as_mut().unwrap();
        for c in path.chars() {
            if c == 'l' {
                assert!(rv.left_tree.is_some());
                rv = rv.left_tree.as_mut().unwrap().borrow_mut();
            }
            else if c == 'r' {
                assert!(rv.right_tree.is_some());
                rv = rv.right_tree.as_mut().unwrap().borrow_mut();
            }
            else if c == '.' {
				return rv;
			}
			else {
                panic!();
            }
        }
        rv
    }

	fn assert_leaf<T: PartialEq + Debug>(node: &mut PointerWaveletTreeNode<T>, elem: &T) {
		assert_node_elem(node, elem, elem);
        assert!(node.left_tree.is_none());
        assert!(node.right_tree.is_none());
		assert_eq!(node.bits.len(), 0);
	}

	fn assert_node<T: PartialEq + Display + Debug>(node: &mut PointerWaveletTreeNode<T>, pattern: &Vec<u32>, min: &T, max: &T) {
		assert_node_elem(node, min, max);
		assert_bit_pattern(node, pattern);
	}

    fn assert_node_elem<T: PartialEq + Debug>(node: &PointerWaveletTreeNode<T>, min: &T, max: &T) {
        assert_eq!(node.min_element, *min);
        assert_eq!(node.max_element, *max);
    }

    fn assert_bit_pattern<T: Display>(node: &mut PointerWaveletTreeNode<T>, pattern: &Vec<u32>) {
        println!("Bitvec: {:?}", node.bits);
        let mut i = 0;
        assert_eq!(node.bits.len() as usize, pattern.len());
        for bit in pattern.iter() {
            assert_eq!(node.bits.get(i), *bit == 1, "Bit {} should be {} since symbol {} is (not) in alphabet {}-{}.", i, *bit, 'x', node.min_element, node.max_element);
            i += 1;
        }
    
    }

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
    fn constructor_empty_data(){
	let mut data: Vec<u32> = Vec::new();
	let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
	let empty_node = Option::None;
	assert_eq!(tree.root, empty_node);
    }

    //Tests if the creation with non-empty data is functional
    //specifically, if the bit vector is initialized correctly
    #[test]
    fn constructor_non_empty_data(){
	let mut data: Vec<u32> = Vec::new();
        data.push(4);
        data.push(2);
        data.push(4);
        data.push(2);
        data.push(4);
	let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
	let mut test_bits = BitVec::new_fill(false, 5);
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


