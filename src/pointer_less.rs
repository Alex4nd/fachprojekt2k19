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

    pub fn new_fill(data: &[T]) {
        
	}

    fn access_rec(&self, index: u32, iteration: u32, l: u32, r: u32, alphL: u32, alphR: u32) -> Option<T> {
        if alphL != alphR {
            let newIndex = 0;
            let newL = 0;
            let newR = 0;
            let newAlphL = 0;
            let newAlphR = 0;
            let result = PointerlessWaveletTree::access_rec(&self, newIndex, iteration+1, newL, newR, newAlphL, newAlphR);
            return result;
        }
        return Option::Some(self.alphabet[alphL as usize].clone());
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



//    ACCESS  Beispiel: abacad

//    INPUT: i=3, Alphabet = [ab|cd], l=1, r=6 (Alphabetlänge), #iteration = 1
//    constant: n=6 (data_size), B[000101|0100|01]

//    1. iteration:
//    Alphabet[ab|cd]
//    l=1, r=6
//    B[3] = 0 -> linkes Kind

//    //child select
//    Alphabet(new) = [a|b]
//    l(new) = n + l = 6 + 1 = 7
//    r(new) = n + l + #(0)(l-r) = 6 + 1 + #(0)(1-6) = 7 + 4

//    i(new) = n*#iteration + #(0)(1-i) = r + #(0)(1-3) = 6*1 + 3 = 9

//    2. iteration:

//    B[9] = 0 -> linkes Kind
//    Alphabet(new) = [a]
//    return a;

