//! Binary indexed data

struct FenwickTree<T> {
    size: usize,
    data: Box<[T]>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Copy + std::ops::AddAssign + std::ops::Sub<Output = T>,
{
    fn new(size: usize, e: T) -> Self {
        Self {
            size,
            data: vec![e; size + 1].into(),
            e,
        }
    }
    fn clear(&mut self) {
        self.data[1..].fill(self.e);
    }
    fn add(&mut self, mut i: usize, x: T) {
        assert!(i <= self.size);
        i += 1;
        while i <= self.size {
            self.data[i] += x;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix_sum(&self, mut i: usize) -> T {
        assert!(i <= self.size);
        let mut ret = self.e;
        while i != 0 {
            ret += self.data[i];
            i &= i - 1;
        }
        ret
    }
    fn range_sum(&self, l: usize, r: usize) -> T {
        self.prefix_sum(r) - self.prefix_sum(l)
    }
}
