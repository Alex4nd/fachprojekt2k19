extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
use num_traits::cast::NumCast;
// use std::convert::TryFrom;

use std::fmt::Display;
use std::fmt::Debug;

use crate::traits::WaveletTree;

pub struct PointerlessWaveletTree<T> {
    alphabet: Vec<T>,
    data_size: u32,
    bits: BitVec<u8>,
}

impl<T: Ord + PartialEq + Clone + Debug + Display + Div<Output = T> + Add<Output = T> + NumCast + Copy> PointerlessWaveletTree<T> {

    pub fn new_fill(data: &[T])  -> PointerlessWaveletTree<T> {

    if data.len() == 0 {
        let mut tree = PointerlessWaveletTree {
            alphabet: Vec::new(),
            data_size: 0,
            bits: BitVec::new(),
        };
        tree
    } else {
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
            let n = data.len() * f64::log2(alphabet.len() as f64).ceil() as usize;
            let mut bits: BitVec<u8> = BitVec::new_fill(true, n as u64);
            let mut bit_length: Vec<u8> = Vec::new(); // length of the vector = supposed length of 'bits'
            PointerlessWaveletTree::initialize_bits(&mut bits, data.len(), &alphabet, 0, alphabet.len() as u32 -1,
                                                                            &data, 0, data.len()-1, &mut bit_length);

            bits.truncate(bit_length.len() as u64);
            print!("bits: \n");                                                                                                      //////DEBUG
            for x in 0..bits.len() {
                print!("{}, ",bits[x]);
            }
            print!("\n");
            let mut tree = PointerlessWaveletTree {
                alphabet: alphabet,
                data_size: data.len() as u32,
                bits: bits,
            };
            tree
        }
    }

    fn initialize_bits(bits: &mut BitVec<u8>, data_size: usize, alphabet: &Vec<T>, alph_l: u32, alph_r: u32, data: &[T], start: usize, end: usize, bit_length: &mut Vec<u8>) {
        if alph_l < alph_r {
            let alph_split_pos = alph_l + 2_u32.pow(((f64::log2((alph_r - alph_l+1) as f64)).ceil() as u32) - 1);
            let mut data_l: Vec<T> = Vec::new();
            let mut data_r: Vec<T> = Vec::new();
            let mut i_help = 0;
            let mut bit: bool;
            for i in start..end + 1 {
                bit = true;
                for j in alph_l..alph_split_pos {
                    if data[i_help] == *alphabet.get(j as usize).unwrap() {
                        bit = false;
                        break;
                    }
                }
                if !bit {
                    data_l.push(data[i_help]);
                } else {
                    data_r.push(data[i_help]);
                }
                bits.set_bit(i as u64, bit);
                bit_length.push(0);
                
                i_help += 1;
            }

            let new_level;
            // #members in the old range = #alphabet - 1
            // one element was cut off and needs to be acknowledged to not mess up the new index
            if (end - start + 1) as usize == alphabet.len() - 1 {
                new_level = data_size-1;
            }
            else {
                new_level = data_size;
            }

            // ZEIGE AUF DAS LINKE KIND
            PointerlessWaveletTree::initialize_bits(bits, data_size, &alphabet, alph_l, alph_split_pos-1,
                                                        &data_l, new_level+start, new_level+start+data_l.len()-1, bit_length);
            // ZEIGE AUF DAS RECHTE KIND
            PointerlessWaveletTree::initialize_bits(bits, data_size, &alphabet, alph_split_pos, alph_r,
                                                        &data_r, new_level+start+data_l.len(), new_level+end, bit_length);
     } else {
         return;
     }
}

    fn access_rec(&self, index: u32, iteration: u32, l: u32, r: u32, alph_l: u32, alph_r: u32) -> Option<T> {
        if alph_l + 1 < alph_r {
            let mut new_index;
            let new_l;
            let new_r;
            let new_alph_l;
            let new_alph_r;

            // FIND WHERE TO SPLIT THE ALPHABET
            let alph_split_pos = alph_l + 2_u32.pow( ((f64::log2((alph_r - alph_l+1) as f64) ).ceil() as u32) - 1);

            let new_level;
            // members in the old range = alphabet size - 1
            // one element was cut off and needs to be acknowledged to not mess up the new index
            if (r - l + 1) as usize == &self.alphabet.len() - 1 {
                new_level = self.data_size*iteration-2;
            }
            else {
                new_level = self.data_size*iteration-1;
            }

            if &self.bits[index as u64] == &false {
                // BITMAP CONTAINS 0 AT POSISTION index
                new_index = new_level + PointerlessWaveletTree::number_of(&self, l, index, &false);
                new_l = self.data_size + l;
                new_r = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false) - 1;
                new_alph_l = alph_l;
                new_alph_r = alph_split_pos - 1;
            } else {
                // BITMAP CONTAINS 1 AT POSISTION index
                new_index = new_level + PointerlessWaveletTree::number_of(&self, l, r, &false)
                                      + PointerlessWaveletTree::number_of(&self, l, index, &true);
                new_l = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false);
                new_r = new_l + PointerlessWaveletTree::number_of(&self, l, r, &true) - 1;
                new_alph_l = alph_split_pos;
                new_alph_r = alph_r;
            }
 
            let result = PointerlessWaveletTree::access_rec(&self, new_index, iteration+1, new_l, new_r, new_alph_l, new_alph_r);
            return result;
        }
        
        if alph_l == alph_r {
            return Option::Some(self.alphabet[alph_l as usize].clone());
        }
        if &self.bits[index as u64] == &false {
            return Option::Some(self.alphabet[alph_l as usize].clone());
        }
        return Option::Some(self.alphabet[alph_r as usize].clone());
    }

    fn select_rec (&self, element: T, index: u32, iteration: u32, l: u32, r: u32, alph_l: u32, alph_r: u32) -> u32 {
        print!("L:  {}, R: {}, index: {}, alph_l: {}, alph_r: {}\n", l, r, index, alph_l, alph_r);                                                                //DEBUG

        if alph_l + 1 < alph_r {
            let mut new_index;
            let new_l;
            let new_r;
            let new_alph_l;
            let new_alph_r;

            // FIND WHERE TO SPLIT THE ALPHABET
            let alph_split_pos = alph_l + 2_u32.pow( ((f64::log2((alph_r - alph_l+1) as f64) ).ceil() as u32) - 1);

            let new_level;
            // members in the old range = alphabet size - 1
            // one element was cut off and needs to be acknowledged to not mess up the new index
            if (r - l + 1) as usize == &self.alphabet.len() - 1 {
                new_level = self.data_size*iteration-2;
            }
            else {
                new_level = self.data_size*iteration-1;
            }

            if self.alphabet[alph_l as usize..alph_split_pos as usize].to_vec().contains(&element){
                // element is in the left "tree"
                new_index = new_level + PointerlessWaveletTree::number_of(&self, l, index, &false);
                new_l = self.data_size + l;
                new_r = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false) - 1;
                new_alph_l = alph_l;
                new_alph_r = alph_split_pos - 1;
                let current_index = PointerlessWaveletTree::select_rec(&self, element, new_index, iteration+1, new_l, new_r, new_alph_l, new_alph_r);
                // its the index of the element in the left child, convert it to the corresponding index in the current tree
                // wikipedia says how
            } else {
                // element is in the right "tree"
                new_index = new_level + PointerlessWaveletTree::number_of(&self, l, r, &false)
                                      + PointerlessWaveletTree::number_of(&self, l, index, &true);
                new_l = self.data_size + l + PointerlessWaveletTree::number_of(&self, l, r, &false);
                new_r = new_l + PointerlessWaveletTree::number_of(&self, l, r, &true) - 1;
                new_alph_l = alph_split_pos;
                new_alph_r = alph_r;
                let current_index = PointerlessWaveletTree::select_rec(&self, element, new_index, iteration+1, new_l, new_r, new_alph_l, new_alph_r);
                // its the index of the element in the right child, convert it to the corresponding index in the current tree
                // wikipedia says how
            }
        }

        // is a leaf!
        // here we are at the bottom of the "tree". proceed here as wikipedia defines the select implementation
        // it needs to return the index of the element we are searching for 

        if self.alphabet[alph_l as usize].clone() == element {
            // element should be the n-th 0, search it with RankSelect.select_0 analogous to the pointer example
            return l; //just the border, not the needed element yet
        } else {
            // element should be the n-th 1, search it with RankSelect.select_1 analogous to the pointer example
            return r; //just the border, not the needed element yet
        }
    }

    // CALCUL NUMBER OF 0's OR 1's IN INTERVALL [l..r]
    fn number_of(&self, l: u32, r: u32, x: &bool) -> u32 {
        let mut result = 0;
        // we dont want to go out of bound, do we?
        let mut bound_r = r+1;
        if r as u64 > self.bits.len() {
            bound_r = self.bits.len() as u32
        }
        for i in l..bound_r {
            if &self.bits[i as u64] == x {
                result += 1 ;
            }
        }
        result
    }

    pub fn deserialize(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        for i in 0..self.data_size {
            result.push(self.access(i as u32).unwrap().clone());
        }
        result
    }

}

impl<T: Ord + PartialEq + Clone + Debug + Display + Div<Output = T> + Add<Output = T> + NumCast + Copy> WaveletTree<T> for PointerlessWaveletTree<T> {

    fn access(&self, index: u32) -> Option<T>{
        if self.data_size == 0 || index >= self.data_size || index < 0 {
            return Option::None
        }
        return PointerlessWaveletTree::access_rec(&self, index, 1, 0, self.data_size-1, 0, self.alphabet.len() as u32-1);
    }

    fn rank(&self, element: T, index: u32) -> u32{
    	return 42;
    }

    fn select(&self, element: T, index: u32) -> u32{
    	if !self.alphabet.contains(&element){
            panic!("Element nicht in Alphabet des Wavelettrees vorhanden")
        }
        if index < 1{
            panic!("Der Index für eine Select anfrage muss größer als 1 sein!")
        }
        if self.bits.len() == 0 {
            panic!("Kein Wavelettree vorhanden");
        }
        return PointerlessWaveletTree::select_rec(&self, element, index, 1, 0, self.data_size-1, 0, self.alphabet.len() as u32 - 1);
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
        let data = vec!   (1,4,1,2,1,5,0,1,0,4,1,0,1,4,1,2,1,5,3,1);

        let pattern = vec!(0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,0,1,0,0,
                           0,0,1,0,0,0,0,0,0,0,0,1,0,1,0,
                           0,1,0,0,1,
                           1,1,1,0,1,0,1,0,1,1,1,1,
                           0,0,1);

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
        test_bits.set_bit(5 as u64, true);
        test_bits.set_bit(6 as u64, true);
	    assert_eq!(tree.bits, test_bits);
    }

    //The position index of the elements in the wavelet tree is assumed to begin at 0

    //Tests the function access with valid parameters.
    //The object at the given index 5 does exist in the wavelet-tree, so the expected output is this object (420)
    #[test]
    fn access_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(2);
        data.push(420);
        data.push(0);
        let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.access(5).unwrap(), 420);
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
        data.push(2);
        data.push(420);
        data.push(0);
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

    //Tests the deserialization
    #[test]
    fn deserialize_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(4);
        data.push(2);
        data.push(4);
        data.push(2);
        data.push(1);
	    let tree: PointerlessWaveletTree<u32> = PointerlessWaveletTree::new_fill(&data[..]);
	    assert_eq!(tree.deserialize(), data);
    }
}
