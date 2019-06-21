pub trait WaveletTree<T> {

    fn access(&self, index: u32) -> Option<T>;

    fn rank(&self, element: T, index: u32) -> u32;

    fn select(&self, element: T, index: u32) -> u32;
}
