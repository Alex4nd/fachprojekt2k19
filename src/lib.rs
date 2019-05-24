extern crate bv;
use bio::data_structures::rank_select::RankSelect;
use bv::BitVec;
use bv::BitsMut;


struct PointerWaveletTree<T> {
    e: T
}

impl<T> PointerWaveletTree<T> {

    fn new() {
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

    #[test]
    fn access() {
    
    }

    #[test]
    fn rank() {
    
    }

    #[test]
    fn select() {
    
    }
}


