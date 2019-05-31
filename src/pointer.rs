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

    //Tests the function access with valid parameters.
    //The object at the given index 4 does exist in the wavelet-tree, so the expected output is this object
    //The position is assumed to begin a 1, not 0
    #[test]
    fn access_success() {
	let mut data: Vec<String> = Vec::new();
	data.push(String::from("Albert"));
	data.push(String::from("Bernd"));
	data.push(String::from("Connor"));
	data.push(String::from("Daria"));
	data.push(String::from("Elena"));
        let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(data);
        let content: String = tree.access(4);
	assert_eq!(content.unwrap(), String::from("Daria"));
    }

    //Tests the function access with an invalid position
    //The object at the given index 6 does not exist in the wavelet-tree, so the expected output is Option::None,
    //to indicate the nonexistence of the object at this position
    //The position is assumed to begin a 1, not 0
    #[test]
    fn access_invalid_position() {
	let mut data: Vec<String> = Vec::new();
	data.push(String::from("Albert"));
	data.push(String::from("Bernd"));
	data.push(String::from("Connor"));
	data.push(String::from("Daria"));
	data.push(String::from("Elena"));
        let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(data);
        let content: String = tree.access(6);
	assert_eq!(content, Option::None);
    }

    //Tests the function rank with valid parameters.
    //The object "1" exists 3 times up to position 5, so the expected output is 3
    //The position is assumed to begin a 1, not 0
    #[test]
    fn rank_success() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.rank(1, 5);
	assert_eq!(3, content);
    }

    //Tests the function rank with an invalid element.
    //The object "42" does not exists in the wavelet tree, so the expected output is 0
    #[test]
    fn rank_invalid_element() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.rank(42, 5);
	assert_eq!(0, content);
    }

    //Tests the function rank with an invalid index, which is too high.
    //The object "1" exists 3 times up to position 5. Although the index is 42, the expected output is 3
    //An index that exceeds the number of objects in the wavelet tree is tolerated and treated as if it's the highest valid index
    //The position is assumed to begin at 1, not 0
    #[test]
    fn rank_position_out_of_bound() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.rank(1, 42);
	assert_eq!(3, content);
    }

    //Tests the function rank with an invalid position of 0.
    //The 0th position of the object "1" does not make sense,
    //so the expected output is 0, to indicate the nonsensical nature of this operation
    //The position is assumed to begin at 1, not 0
    #[test]
    fn rank_position_0() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.rank(1, 0);
	assert_eq!(0, content);
    }

    //Tests the function select with valid parameters.
    //The second occurence of the object "0" exists at position 4 in the wavelet tree, so the expected output is 4
    //The position is assumed to begin at 1, not 0
    #[test]
    fn select_success() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.select(0, 2);
	assert_eq!(4, content);
    }

    //Tests the function select with an invalid element that does not exist in the wavelet tree.
    //The first occurence of the object "42" does not exist in the wavelet tree,
    //so the expected output is 0, to indicate non-existence of the object in the wavleet tree
    //The position is assumed to begin at 1, not 0
    #[test]
    fn select_invalid_element() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.select(42, 1);
	assert_eq!(0, content);
    }

    //Tests the function select with an invalid occurence that does not exist in the wavelet tree.
    //The 4th occurence of the object "1" does not exist in the wavelet tree,
    //so the expected output is 0, to indicate non-existence of the 4th occurence of the object in the wavleet tree
    //The position is assumed to begin at 1, not 0
    #[test]
    fn select_occurence_out_of_bound() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.select(42, 1);
	assert_eq!(0, content);
    }

    //Tests the function select with an invalid occurence of 0.
    //The 0th occurence of the object "1" does not make sense,
    //so the expected output is 0, to indicate the nonsensical nature of this operation
    //The position is assumed to begin at 1, not 0
    #[test]
    fn select_occurence_0() {
	let mut data: Vec<u32> = Vec::new();
    	data.push(1);
	data.push(0);
	data.push(1);
	data.push(0);
	data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(data);
        let content: u32 = tree.select(1, 0);
	assert_eq!(0, content);
    }
}


