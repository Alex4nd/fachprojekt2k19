extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;


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

impl<T> PointerWaveletTree<T> {

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
        PointerWaveletTree::Node {
            leftAlphabet: Vec::new(),
            rightAlphabet: Vec::new(),
            leftTree: Box::new(PointerWaveletTree::Nil),
            rightTree: Box::new(PointerWaveletTree::Nil),
            bits: BitVec::new_fill(false, capacity)
        }
    }

    fn fill_rec(alphabet: Vec<T>, sequence: Vec<T>) -> PointerWaveletTree<T> {
    }
}

impl<T> WaveletTree<T> for PointerWaveletTree<T> {

    fn access(&self, index: u32) {
    
    }

    fn rank(&self, element: T, index: u32) {
    
    }

    fn select(&self, element: T, index: u32) {
    
    }
}



trait WaveletTree<T> {

    fn access(&self, index: u32);

    fn rank(&self, element: T, index: u32);

    fn select(&self, element: T, index: u32);

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn access() {
         let tree: PointerWaveletTree<u32> = PointerWaveletTree::new(64);
         tree.access(5)
    }

    #[test]
    fn rank() {
    
    }

    #[test]
    fn select() {
    
    }
}


