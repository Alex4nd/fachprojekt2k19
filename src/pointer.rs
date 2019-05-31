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

    #[test]
    fn access() {
	let mut data: Vec<String> = Vec::new();
	data.push(String::from("Albert"));
	data.push(String::from("Bernd"));
	data.push(String::from("Connor"));
	data.push(String::from("Daria"));
	data.push(String::from("Elena"));
        let tree: PointerWaveletTree<String> = PointerWaveletTree::new_fill(data);
        let content: String = tree.access(4).unwrap();
	assert_eq!(content, String::from("Daria"));
    }

    #[test]
    fn rank() {
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

    #[test]
    fn select() {
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
}


