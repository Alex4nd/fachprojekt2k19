extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};

use crate::traits::WaveletTree;

pub struct PointerlessWaveletTree<T> {
    alphabet: Vec<T>,
    data_size: i32,
    bits: BitVec<u8>,
}

impl<T: Ord + PartialEq + Clone + Div + Add> PointerlessWaveletTree<T> {

    pub fn new_fill(data: &[T]) {

	}

    fn fill_bits(level: i32, data_size: i32, alphabet: &[T], sequence: &[T]) {

    }
}
