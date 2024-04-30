//! Binary indexed tree

struct BinaryIndexedTree<T, F> {
    size: usize,
    tree: Box<[T]>,
    e: T,
    f: F,
}

impl<T, F> BinaryIndexedTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    fn new(size: usize, e: T, f: F) -> Self {
        Self {
            size,
            tree: vec![e; size + 1].into(),
            e,
            f,
        }
    }
    fn clear(&mut self) {
        self.tree[1..].fill(self.e);
    }
    fn update(&mut self, mut i: usize, x: T) {
        assert!(i <= self.size);
        i += 1;
        while i <= self.size {
            self.tree[i] = (self.f)(self.tree[i], x);
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> T {
        assert!(i <= self.size);
        let mut ret = self.e;
        while i != 0 {
            ret = (self.f)(ret, self.tree[i]);
            i &= i - 1;
        }
        ret
    }
}

#[test]
fn prefix_sum() {
    let data = [2, -1, 3, 0, 3, -7, 10, 9];
    let mut pref = vec![0; data.len() + 1];
    let mut bit = BinaryIndexedTree::new(data.len(), 0, std::ops::Add::add);
    for (i, x) in data.into_iter().enumerate() {
        pref[i + 1] = pref[i] + x;
        bit.update(i, x);
    }
    for (i, x) in pref.into_iter().enumerate() {
        assert_eq!(x, bit.prefix(i));
    }
}

#[test]
fn prefix_max() {
    let data = [2, -1, 3, 0, 3, -7, 10, 9];
    let mut pref = vec![i32::MIN; data.len() + 1];
    let mut bit = BinaryIndexedTree::new(data.len(), i32::MIN, std::cmp::max);
    for (i, x) in data.into_iter().enumerate() {
        pref[i + 1] = pref[i].max(x);
        bit.update(i, x);
    }
    for (i, x) in pref.into_iter().enumerate() {
        assert_eq!(x, bit.prefix(i));
    }
}

#[test]
fn prefix_closure() {
    let data = [2, -1, 3, 0, 3, -7, 10, 9];
    let mut pref = vec![(i32::MAX, i32::MIN); data.len() + 1];
    let mut bit = BinaryIndexedTree::new(data.len(), (i32::MAX, i32::MIN), |l, r| (l.0.min(r.0), l.1.max(r.1)));
    for (i, x) in data.into_iter().enumerate() {
        pref[i + 1] = (pref[i].0.min(x), pref[i].1.max(x));
        bit.update(i, (x, x));
    }
    for (i, x) in pref.into_iter().enumerate() {
        assert_eq!(x, bit.prefix(i));
    }
}
