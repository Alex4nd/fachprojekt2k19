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
		let mut data: Vec<String> = Vec::new();
		data.push(String::from("Albert"));
		data.push(String::from("Bernd"));
		data.push(String::from("Connor"));
		data.push(String::from("Daria"));
		data.push(String::from("Elena"));

	let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(&data[..]);
	let content: String = tree.access(3).unwrap();
	assert_eq!(content, String::from("Daria"));
	}

	//Tests the function access with an invalid position
	//The object at the given index 5 does not exist in the wavelet-tree, so the expected output is Option::None,
	//to indicate the nonexistence of the object at this position
	#[test]
	fn access_invalid_position() {
		let mut data: Vec<String> = Vec::new();
		data.push(String::from("Albert"));
		data.push(String::from("Bernd"));
		data.push(String::from("Connor"));
		data.push(String::from("Daria"));
		data.push(String::from("Elena"));
		let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(&data[..]);
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


