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
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    fn new(size: usize, e: T, f: F) -> Self {
        let size = size.next_power_of_two();
        Self {
            size,
            data: vec![e; size << 1].into(),
            e,
            f,
        }
    }
    fn build(&mut self) {
        for i in (1..self.size).rev() {
            self.data[i] = (self.f)(self.data[i << 1], self.data[i << 1 | 1]);
        }
    }
    fn set(&mut self, mut i: usize, x: T) {
        assert!(i < self.size);
        i += self.size;
        self.data[i] = x;
        while i != 1 {
            i >>= 1;
            self.data[i] = (self.f)(self.data[i << 1], self.data[i << 1 | 1]);
        }
    }
    fn prod(&self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.size);
        let mut prod_l = self.e;
        let mut prod_r = self.e;
        l += self.size;
        r += self.size;
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
    fn all_prod(&mut self) -> T {
        self.data[1]
    }
    fn clear(&mut self) {
        self.data[1..].fill(self.e);
    }
}

#[test]
fn test() {}
