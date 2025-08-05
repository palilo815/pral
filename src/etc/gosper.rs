//! Gosper's Hack
//!
//! - **Author** &emsp; Bill Gosper
//! - **Source** &emsp; [Roseta Code](https://rosettacode.org/wiki/Gosper%27s_hack)
//! - **Update** &emsp; 2025-08-05
//!
//! Gosper's Hack is an algorithm to find the next integer with the same number of set bits (population count).

/// Iterate all combination of certain size.
struct Gosper {
    n: u32,
    mask: u32,
}

impl Gosper {
    fn new(n: u32, k: u32) -> Self {
        Self { n, mask: (1 << k) - 1 }
    }
}

impl Iterator for Gosper {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.mask >= 1 << self.n {
            return None;
        }
        let x = self.mask;
        let c = x & x.wrapping_neg();
        let r = x + c;
        let y = (((r ^ x) >> 2) / c) | r;
        self.mask = y;
        Some(x)
    }
}
