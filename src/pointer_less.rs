/*extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};

use crate::traits::WaveletTree;

pub struct PointerlessWaveletTree<'a, T> {
    alphabet: Vec<T>,
    data_size: i32,
    bits: BitVec<u8>,
}

impl<'a, T: Ord + PartialEq + Clone + Div + Add> PointerlessWaveletTree<'a, T> {

    pub fn new_fill(data: &[T]) -> PointerlessWaveletTree<'a, T> {
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
        alphabet.sort();
        let tree = PointerlessWaveletTree {
            alphabet: alphabet,
            data_size: data.len();
            bits: BitVec::new_fill(false, 32);,
        };
        //FÜLLE BITMAP
        fill_bits(0, tree.data_size, &tree.alphabet[..], data);
        tree
    }

    pub fn fill_bits(level: i32, data_size: i32, alphabet: &'a [T], sequence: &[T]) {

        if alphabet.len() > 1 {

            // START MUSS TROTZDEM ANGEPASST WERDEN!!!!
            let mut start = data_size * ( level - 1 );
            let exp = f64::ceil( f64::log2(alphabet.len() as f64) );
            let middle = 2i8.pow( exp as u32 - 1 );

            for elem in sequence.iter() {
                let mut pos_in_alpha: usize = 0;
                for alph in alphabet.iter() {
                    if elem == alph {
                        if pos_in_alpha < middle {
                            bits.set_bit(start as u64, false);
                        }
                        else {
                            bits.set_bit(start as u64, true);
                        }
                        start += 1;
                        break;
                    }
                    pos_in_alpha += 1;
                }

            }
            // CREATE SUB_SEQ FOR LEFT AND RIGHT CHILD
            let mut data_left: Vec<T> = Vec::new();
            let mut data_right: Vec<T> = Vec::new();
            let mut index: usize = 0;
            while index < data.len() {
                if(!bits.get_bit(index)) {
                    data_left.push( data[index] );
                } else {
                    data_right.push( data[index] );
                }
                index += 1;
            }

            PointerlessWaveletTree::fill_bits(level+1, data_size, &alphabet[.. middle as usize], data_left);
            PointerlessWaveletTree::fill_bits(level+1, data_size, &alphabet[middle as usize ..], data_right);

        } else {
            //TODO
            // WIE SOLLEN DIE CHARS GESPEICHERT WERDEN!
        }
    }
}


impl<'a, T> WaveletTree<T> for PointerlessWaveletTree<'a, T> {

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

    //TODO


}
*/
