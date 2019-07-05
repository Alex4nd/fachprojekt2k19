extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;

use std::ops::{Add, Div};
use num_traits::cast::NumCast;
use num_traits::cast::cast;
// use std::convert::TryFrom;

use std::fmt::Display;
use std::fmt::Debug;

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

impl<T: Ord + PartialEq + Clone + Debug + Display + Div<Output = T> + Add<Output = T> + NumCast + Copy> PointerWaveletTree<T> {

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

            // let exp = f32::ceil( f32::log2(alphabet.len() as f32) ) as usize;
            // let middle = 2i8.pow( exp as u32 - 1 ) as usize;
            let middle = (alphabet[0] + alphabet[alphabet.len() - 1]) / cast(2).unwrap();
            println!("\tmiddle {:?}", middle);

            let mut length = 0;
            for elem in sequence.iter() {
                let mut position: usize = 0;
                for alph in alphabet.iter() {
                    if elem == alph {
                        println!("[{}] symbol {} in alphabet {:?} -> {}", length, elem, alphabet, if *elem > middle {"right"} else {"left"});
                        if *elem <= middle {
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

            let mut index_middle = 0;
            for elem in alphabet.iter() {
                if *elem > middle {
                    break;
                }
                index_middle += 1;
            }

            println!("recusrive fill:\n\tleft: {:?}\n\tright: {:?}\n", &alphabet[.. index_middle], &alphabet[index_middle ..]);
            PointerWaveletTreeNode {
                left_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[.. index_middle], &sequence))),
                right_tree: Option::Some(Box::new(PointerWaveletTree::fill_rec(&alphabet[index_middle ..], &sequence))),
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

    pub fn deserialize(&self) -> Vec<T> {
        let mut result: Vec<T> = Vec::new();
        let data_size = &self.root.as_ref().unwrap().bits.len();
        for i in 0..*data_size {
            result.push(self.access(i as u32).unwrap().clone());
        }
        result
    }

    pub fn to_vec(&mut self) -> Option<Vec<T>> {
        if self.root.is_none() {
            return Option::None;
        }

        return Option::Some(self.root.as_mut().unwrap().to_vec());
    }
}

impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> PointerWaveletTreeNode<T> {
    fn to_vec(&mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.bits.len() as usize);

        if self.is_leaf() {
            vec.push(Clone::clone(&self.min_element));
        } 
        else { 
        
            let left = self.left_tree.as_mut().unwrap();
            let right = self.right_tree.as_mut().unwrap();

            let vecl = left.to_vec();
            let vecr = right.to_vec();

            let emptyl = left.is_leaf();
            let emptyr = right.is_leaf();

            let mut iterl = vecl.iter();
            let mut iterr = vecr.iter(); 

            for i in 0..self.bits.len() {
                if self.bits.get(i) {
                    if emptyr {
                        vec.push(Clone::clone(&right.min_element));
                    }
                    else {
                        vec.push(Clone::clone(iterr.next().unwrap()));
                    }
                }
                else {
                    if emptyl {
                        vec.push(Clone::clone(&left.min_element));
                    }
                    else {
                        vec.push(Clone::clone(iterl.next().unwrap()));
                    }
                }
            }
        }
        return vec
    }
}


impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> Node<T> for PointerWaveletTreeNode<T> {

    fn is_leaf(&self) -> bool {
        self.left_tree.is_none() && self.right_tree.is_none()
    }
    
    
    fn access(&self, index: u32) -> T{
        if self.min_element == self.max_element{
            return self.min_element;
        }

        let rs = RankSelect::new(self.bits.clone(),1);
        if !self.bits[index as u64] {
            let result = rs.rank_0(index as u64).unwrap() as u32 - 1;
            self.left_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                    .access(result)
        }
        else{
            let result = rs.rank_1(index as u64).unwrap() as u32 - 1    ;
            self.right_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                    .access(result)
        }
        
    }
    

    fn rank(&self, element: T, index: u32) -> u32 {
        if &self.min_element == &self.max_element {
            return index + 1;
        }

        let rs = RankSelect::new(self.bits.clone(),1);
        if  element <= (self.min_element + self.max_element) / cast(2).unwrap() {
            let result = rs.rank_0(index as u64);
            match result{
                Some(0) => {
                    0
                },
                
                Some(idx) => {
                    self.left_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                        .rank(element, (idx - 1) as u32)
                },
                
                None => panic!("Invalider Indexwert für binäres Rank"),
            }
        }
        else{
            let result = rs.rank_1(index as u64);
            match result{
                Some(0) => {
                    0
                },
                
                Some(idx) => {
                    self.right_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                        .rank(element, (idx - 1) as u32)
                },
                
                None => panic!("Invalider Indexwert für binäres Rank"),
            }
        }
    }


    fn select(&self, element: T, index: u32) -> u32{
        if self.is_leaf() {
            return index - 1;
        }

        let rs = RankSelect::new(self.bits.clone(),1);
        let result;

        if element <= (self.min_element + self.max_element) / cast(2).unwrap() {
            let sel = &self.left_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                .select(element, index);
            result = rs.select_0((sel + 1) as u64);
        }
        else{
            let sel = &self.right_tree.as_ref().expect("Der Baum sollte hier einen Knoten haben, hat aber keinen")
                .select(element, index);
            result = rs.select_1((sel + 1) as u64);
        }
        return result.expect("Invalider Indexwert für binäres Select") as u32;
    }
}

impl<T: Ord + PartialEq + Clone + Div<Output = T> + Add<Output = T> + NumCast + Copy> WaveletTree<T> for PointerWaveletTree<T> {

    fn access(&self, index: u32) -> std::option::Option<T>{
        let root = &self.root.as_ref().expect("Kein Wavelettree vorhanden");
        if index >= root.bits.len() as u32 {
            None
        }
        else{
            Some(root.access(index))
        }
    }
    
    fn rank(&self, element: T, index: u32) -> u32 {
        if !self.alphabet.contains(&element){
            //panic!("Element nicht in Alphabet des Wavelettrees vorhanden")
            return 0;
        }
        self.root.as_ref().expect("Kein Wavelettree vorhanden")
            .rank(element, index)
    }

    fn select(&self, element: T, index: u32) -> u32{
    
        if !self.alphabet.contains(&element) {
            panic!("Element nicht in Alphabet des Wavelettrees vorhanden")
        }
        if index < 1 {
            panic!("Der Index für eine Select anfrage muss größer als 1 sein!")
        }
        self.root.as_ref().expect("Kein Wavelettree vorhanden")
            .select(element, index)
    }

}

impl<T: Ord + PartialEq> PointerWaveletTreeNode<T> {
    fn is_leaf(&self) -> bool {
        return self.min_element == self.max_element;
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
    fn to_vec() {
        let data = vec!(1,4,1,2,1,5,0,1,0,4,1,0,1,4,1,2,1,5,3,1);

        let mut tree = PointerWaveletTree::new_fill(&data);

        assert_eq!(tree.to_vec(), Some(data));
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
        data.push(b'b');
        data.push(b'm');
        data.push(b'c');
        data.push(b'x');
        data.push(b'd');
        data.push(b'e');
        data.push(b'z');
        data.push(b'x');
        let tree: PointerWaveletTree<u8> = PointerWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.access(3), Some(b'm'), "Access at index 3 should return m");
        assert_eq!(tree.rank(b'b',3), 2, "Rank b with index 3 should return 2");
        assert_eq!(tree.rank(b'x',7), 1, "Rank x with index 7 should return 1");
        assert_eq!(tree.select(b'e',1), 7, "Select e with count 1 should return 7");
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
        data.push(10);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.access(0), Some(1));
        assert_eq!(tree.access(1), Some(0));
        assert_eq!(tree.access(2), Some(1));
        assert_eq!(tree.access(3), Some(0));
        assert_eq!(tree.access(4), Some(1));
        assert_eq!(tree.access(5), Some(10));
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
        assert_eq!(tree.access(5), Option::None);
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
        assert_eq!(tree.rank(1, 2), 2);
        assert_eq!(tree.rank(1, 4), 3);
        assert_eq!(tree.rank(0, 2), 1);
        assert_eq!(tree.rank(0, 4), 2);
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
        assert_eq!(tree.rank(42, 4), 0);
    }

    //Tests the function rank with an invalid position index, which is too high
    //The object "1" exists 3 times up to position index 4, but index 5 is not defined, so it panics
    #[test]
    #[should_panic]
    fn rank_position_out_of_bound() {
        let mut data: Vec<u32> = Vec::new();
        data.push(1);
        data.push(0);
        data.push(1);
        data.push(0);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        let content: u32 = tree.rank(1, 5);
        assert_eq!(content, 3);
        //panic goes here
    }
    #[test]
    fn rank_success_complex() {
        //              0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9
        let data = vec!(1,4,1,2,1,7,0,1,0,4,1,0,1,4,1,2,1,7,3,1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.rank(1, 0), 1);
        assert_eq!(tree.rank(1, 1), 1);
        assert_eq!(tree.rank(1, 2), 2);
        assert_eq!(tree.rank(1, 3), 2);
        assert_eq!(tree.rank(1, 4), 3);
        assert_eq!(tree.rank(1, 5), 3);
        assert_eq!(tree.rank(1, 6), 3);
        assert_eq!(tree.rank(1, 7), 4);
        assert_eq!(tree.rank(1, 8), 4);
        assert_eq!(tree.rank(1, 9), 4);
        assert_eq!(tree.rank(1, 10), 5);
        assert_eq!(tree.rank(1, 11), 5);
        assert_eq!(tree.rank(1, 12), 6);
        assert_eq!(tree.rank(1, 13), 6);
        assert_eq!(tree.rank(1, 14), 7);
        assert_eq!(tree.rank(1, 15), 7);
        assert_eq!(tree.rank(1, 16), 8);
        assert_eq!(tree.rank(1, 17), 8);
        assert_eq!(tree.rank(1, 18), 8);
        assert_eq!(tree.rank(1, 19), 9);

        assert_eq!(tree.rank(7, 0), 0);
        assert_eq!(tree.rank(7, 1), 0);
        assert_eq!(tree.rank(7, 2), 0);
        assert_eq!(tree.rank(7, 3), 0);
        assert_eq!(tree.rank(7, 4), 0);
        assert_eq!(tree.rank(7, 5), 1);
        assert_eq!(tree.rank(7, 6), 1);
        assert_eq!(tree.rank(7, 7), 1);
        assert_eq!(tree.rank(7, 8), 1);
        assert_eq!(tree.rank(7, 9), 1);
        assert_eq!(tree.rank(7, 10), 1);
        assert_eq!(tree.rank(7, 11), 1);
        assert_eq!(tree.rank(7, 12), 1);
        assert_eq!(tree.rank(7, 13), 1);
        assert_eq!(tree.rank(7, 14), 1);
        assert_eq!(tree.rank(7, 15), 1);
        assert_eq!(tree.rank(7, 16), 1);
        assert_eq!(tree.rank(7, 17), 2);
        assert_eq!(tree.rank(7, 18), 2);
        assert_eq!(tree.rank(7, 19), 2); 
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
        assert_eq!(tree.select(0, 1), 1);
        assert_eq!(tree.select(0, 2), 3);
        assert_eq!(tree.select(1, 1), 0);
        assert_eq!(tree.select(1, 2), 2);
        assert_eq!(tree.select(1, 3), 4);
    }

    #[test]
    fn select_success_complex() {
        //              0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9
        let data = vec!(1,4,1,2,1,7,0,1,0,4,1,0,1,4,1,2,1,7,3,1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.select(0, 1), 6);
        assert_eq!(tree.select(0, 2), 8);
        assert_eq!(tree.select(0, 3), 11);

        assert_eq!(tree.select(1, 1), 0);
        assert_eq!(tree.select(1, 2), 2);
        assert_eq!(tree.select(1, 3), 4);
        assert_eq!(tree.select(1, 4), 7);
        assert_eq!(tree.select(1, 5), 10);
        assert_eq!(tree.select(1, 6), 12);
        assert_eq!(tree.select(1, 7), 14);
        assert_eq!(tree.select(1, 8), 16);
        assert_eq!(tree.select(1, 9), 19);

        assert_eq!(tree.select(7, 1), 5);
        assert_eq!(tree.select(7, 2), 17);
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

    //Tests the deserialization
    #[test]
    fn deserialize_success() {
        let mut data: Vec<u32> = Vec::new();
        data.push(4);
        data.push(2);
        data.push(4);
        data.push(2);
        data.push(1);
        let tree: PointerWaveletTree<u32> = PointerWaveletTree::new_fill(&data[..]);
        assert_eq!(tree.deserialize(), data);
    }
}


