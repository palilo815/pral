struct LazySeg<T, U, F1, F2, F3> {
    size: usize,
    height: u32,
    tree: Box<[T]>,
    lazy: Box<[U]>,
    e: T,
    off: U,
    op: F1,
    mapping: F2,
    composition: F3,
}

impl<T, U, F1, F2, F3> std::ops::Index<usize> for LazySeg<T, U, F1, F2, F3> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.tree[i + self.size]
    }
}

impl<T, U, F1, F2, F3> std::ops::IndexMut<usize> for LazySeg<T, U, F1, F2, F3> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.tree[i + self.size]
    }
}

impl<T, U, F1, F2, F3> LazySeg<T, U, F1, F2, F3>
where
    T: Copy,
    U: Copy,
    F1: Fn(T, T) -> T,
    F2: Fn(&mut T, U),
    F3: Fn(&mut U, U),
{
    fn new(size: usize, e: T, off: U, op: F1, mapping: F2, composition: F3) -> Self {
        let size = size.next_power_of_two();
        let height = size.trailing_zeros() + 1;
        LazySeg {
            size,
            height,
            tree: vec![e; size << 1].into(),
            lazy: vec![off; size].into(),
            e,
            off,
            op,
            mapping,
            composition,
        }
    }
    fn build(&mut self) {
        (1..self.size).rev().for_each(|i| self._pull(i));
    }
    fn apply(&mut self, mut l: usize, mut r: usize, f: U) {
        assert!(l <= r && r <= self.size);
        if l == r {
            return;
        }
        l += self.size;
        r += self.size;
        let l0 = l;
        let r0 = r;
        let anc_l = (l.trailing_zeros() + 1..self.height).map(|i| l0 >> i);
        let anc_r = (r.trailing_zeros() + 1..self.height).map(|i| (r0 - 1) >> i);
        anc_l.clone().rev().for_each(|i| self._push(i));
        anc_r.clone().rev().for_each(|i| self._push(i));
        while l != r {
            if l & 1 == 1 {
                self._all_apply(l, f);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                self._all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
        anc_l.for_each(|i| self._pull(i));
        anc_r.for_each(|i| self._pull(i));
    }
    fn product(&mut self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r && r <= self.size);
        if l == r {
            return self.e;
        }
        l += self.size;
        r += self.size;
        (l.trailing_zeros() + 1..self.height).rev().for_each(|i| self._push(l >> i));
        (r.trailing_zeros() + 1..self.height).rev().for_each(|i| self._push((r - 1) >> i));
        let mut res = (self.e, self.e);
        while l != r {
            if l & 1 == 1 {
                res.0 = (self.op)(res.0, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res.1 = (self.op)(self.tree[r], res.1);
            }
            l >>= 1;
            r >>= 1;
        }
        (self.op)(res.0, res.1)
    }
    fn _pull(&mut self, i: usize) {
        self.tree[i] = (self.op)(self.tree[i << 1], self.tree[i << 1 | 1]);
    }
    fn _push(&mut self, i: usize) {
        self._all_apply(i << 1, self.lazy[i]);
        self._all_apply(i << 1 | 1, self.lazy[i]);
        self.lazy[i] = self.off;
    }
    fn _all_apply(&mut self, i: usize, f: U) {
        (self.mapping)(&mut self.tree[i], f);
        if i & self.size == 0 {
            (self.composition)(&mut self.lazy[i], f);
        }
    }
}
