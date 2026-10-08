use std::fmt::{self, Display};
pub struct BloomFilter<T, const M: usize> {
    hashs: Vec<fn(&T) -> usize>,
    bits: [bool; M],
}

impl<T, const M: usize> BloomFilter<T, M> {
    pub fn new(hashs: Vec<fn(&T) -> usize>) -> Self {
        BloomFilter {
            hashs,
            bits: [false; M],
        }
    }

    pub fn k(&self) -> usize {
        self.hashs.len()
    }

    pub fn m(&self) -> usize {
        M
    }

    pub fn add(&mut self, e: &T) {
        for f in &self.hashs {
            self.bits[f(e) % M] = true;
        }
    }

    pub fn check(&self, e: &T) -> bool {
        for f in &self.hashs {
            if !self.bits[f(e) % M] {
                return false;
            }
        }
        true
    }
}

impl <T, const M: usize> Display for BloomFilter<T, M>  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BloomFilter(k:{}, m:{})", self.k(), self.m())
    }
}
