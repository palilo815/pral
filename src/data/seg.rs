//! Segment data

struct SegmentTree<T, F> {
    size: usize,
    data: Box<[T]>,
    e: T,
    f: F,
}

impl<T, F> std::ops::Index<usize> for SegmentTree<T, F> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i + self.size]
    }
}
impl<T, F> std::ops::IndexMut<usize> for SegmentTree<T, F> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.data[i + self.size]
    }
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    fn new(size: usize, e: T, f: F) -> Self {
        let size = size.next_power_of_two();
        let data = vec![e; size << 1].into_boxed_slice();
        Self { size, data, e, f }
    }
    fn build(&mut self) {
        (1..self.size).rev().for_each(|i| self._pull(i));
    }
    fn set(&mut self, mut i: usize, x: T) {
        assert!(i < self.size);
        i += self.size;
        self.data[i] = x;
        while i != 1 {
            i >>= 1;
            self._pull(i);
        }
    }
    fn prod(&self, range: std::ops::Range<usize>) -> T {
        assert!(range.start <= range.end && range.end <= self.size);
        let mut l = self.size + range.start;
        let mut r = self.size + range.end;
        let mut prod_l = self.e;
        let mut prod_r = self.e;
        while l != r {
            if l & 1 == 1 {
                prod_l = (self.f)(prod_l, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                prod_r = (self.f)(self.data[r], prod_r);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.f)(prod_l, prod_r)
    }
    #[inline]
    fn _pull(&mut self, i: usize) {
        self.data[i] = (self.f)(self.data[i << 1], self.data[i << 1 | 1]);
    }
}

#[test]
fn test() {}
