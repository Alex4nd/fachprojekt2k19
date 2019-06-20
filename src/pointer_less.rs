extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
// use std::convert::TryFrom;

use std::fmt::Display;
use std::fmt::Debug;

use crate::traits::WaveletTree;

pub struct PointerlessWaveletTree<T> {
    alphabet: Vec<T>,
    data_size: u32,
    bits: BitVec<u8>,
}

impl<T: Ord + PartialEq + Clone + Div + Add> PointerlessWaveletTree<T> {

    pub fn new_fill(data: &[T])  -> PointerlessWaveletTree<T> {
        let mut tree = PointerlessWaveletTree {
            alphabet: Vec::new(),
            data_size: data.len() as u32,
            bits: BitVec::new(),
        };
        tree
	}

    fn access_rec(&self, index: u32, iteration: u32, l: u32, r: u32, alph_l: u32, alph_r: u32) -> Option<T> {
        if alph_l != alph_r {
            let mut new_index;
            let mut new_l;
            let mut new_r;
            let mut new_alph_l;
            let mut new_alph_r;

            // FIND WHERE TO SPLIT THE ALPHABET
            let alph_split_pos = alph_l + 2_u32.pow( ((f64::log2((alph_r - alph_l+1) as f64) ).ceil() as u32) - 1);

            if &self.bits[index as u64] == &false {
                // BITMAP CONTAINS 0 AT POSISTION index
                new_index = self.data_size*iteration + PointerlessWaveletTree::number_of(&self, l, index, &false);
                new_l = self.data_size + l;
                new_r = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false) - 1;
                new_alph_l = alph_l;
                new_alph_r = alph_split_pos - 1;
            } else {
                // BITMAP CONTAINS 0 AT POSISTION index
                new_index = self.data_size*iteration + PointerlessWaveletTree::number_of(&self, l, r, &false)
                                                     + PointerlessWaveletTree::number_of(&self, l, index, &true) ;
                new_l = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false);
                new_r = new_l + PointerlessWaveletTree::number_of(&self, l, r, &true) - 1;
                new_alph_l = alph_split_pos;
                new_alph_r = alph_r;
            }
            let result = PointerlessWaveletTree::access_rec(&self, new_index, iteration+1, new_l, new_r, new_alph_l, new_alph_r);
            return result;
        }
        return Option::Some(self.alphabet[alph_l as usize].clone());
    }

    // CALCUL NUMBER OF 0's OR 1's IN INTERVALL [l..r]
    fn number_of(&self, l: u32, r: u32, x: &bool) -> u32 {
        let mut result = 0;
        for i in l..r {
            if &self.bits[i as u64] == x {
                result += 1 ;
            }
        }
        result
    }
}


impl<T: Ord + PartialEq + Clone + Debug + Display + Div + Add> WaveletTree<T> for PointerlessWaveletTree<T> {

    fn access(&self, index: u32) -> Option<T>{
        if self.data_size == 0 {
            return Option::None
        }
        return PointerlessWaveletTree::access_rec(&self, index, 1, 1, self.data_size, 1, self.alphabet.len() as u32);
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
        let alphabet = vec!(0,1,2,3,4,5);

        let tree = PointerlessWaveletTree::new_fill(&data);

        assert_eq!(alphabet, tree.alphabet);
    }

    #[test]
    fn constructor_data() {
        // aiabar a ia aiabarda
        // 14121501041014121531
        let data = vec!(1,4,1,2,1,5,0,1,0,4,1,0,1,4,1,2,1,5,3,1);

        let pattern = vec!(0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,1,0,
                           0,0,1,0,0,0,0,0,0,0,0,1,0,0,
                           0,1,0,0,1,0,
                           1,1,1,0,1,0,1,0,1,1,1,1,
                           1,1,1,0);

        let mut pattern_bool = Vec::new();
        for x in pattern.iter() {
            if *x == 0 {
                pattern_bool.push(false);
            }
            else {
                pattern_bool.push(true);
            }
        }

        let bitvec = BitVec::from_bits(pattern_bool);
        let tree = PointerlessWaveletTree::new_fill(&data);

        assert_eq!(bitvec, tree.bits);
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
        let tree: PointerlessWaveletTree<u8> = PointerlessWaveletTree::new_fill(&data[..]);
        let content = tree.access(3).unwrap();
        assert_eq!(content, b'd');
    }

    //Tests if the creation with empty data is functional, assuming the function is used to generate an empty alphabet, data_size of 0 and an empty bit vector
    #[test]
    fn constructor_empty_data(){
	    let mut data: Vec<u32> = Vec::new();
	    let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
	    assert_eq!(tree.alphabet, Vec::new());
        assert_eq!(tree.data_size, 0);
        assert_eq!(tree.bits, BitVec::new());
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
        data.push(1);
	    let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
	    let mut test_bits = BitVec::new_fill(false, 8);
	    test_bits.set_bit(0 as u64, true);
	    test_bits.set_bit(2 as u64, true);
        test_bits.set_bit(6 as u64, true);
        test_bits.set_bit(7 as u64, true);
	    assert_eq!(tree.bits, test_bits);
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
        data.push(2);
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
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
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.select(1, 0);
    }
}
