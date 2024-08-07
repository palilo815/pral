//! Binary indexed tree

struct BinaryIndexedTree<T, F> {
    size: usize,
    data: Box<[T]>,
    e: T,
    f: F,
}

impl<T, F> BinaryIndexedTree<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    fn new(size: usize, e: T, f: F) -> Self {
        let data = vec![e; size].into_boxed_slice();
        Self { size, data, e, f }
    }
    fn from(data: Vec<T>, e: T, f: F) -> Self {
        let size = data.len();
        let mut data = data.into_boxed_slice();
        for i in 1..size + 1 {
            let j = i + (i & i.wrapping_neg()) - 1;
            if j < size {
                data[j] = f(data[i - 1], data[j]);
            }
        }
        Self { size, data, e, f }
    }
    fn update(&mut self, mut i: usize, v: T) {
        assert!(i <= self.size);
        i += 1;
        while i <= self.size {
            self.data[i - 1] = (self.f)(v, self.data[i - 1]);
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> T {
        assert!(i <= self.size);
        let mut ret = self.e;
        while i != 0 {
            ret = (self.f)(ret, self.data[i - 1]);
            i &= i - 1;
        }
        ret
    }
    fn partition_point<P: Fn(T) -> bool>(&self, pred: P) -> (usize, T) {
        let mut i = 0;
        let mut acc = self.e;
        let mut len = 1 << self.size.ilog2();
        while len != 0 {
            if i + len <= self.size && pred((self.f)(acc, self.data[i + len - 1])) {
                i += len;
                acc = (self.f)(acc, self.data[i - 1]);
            }
            len >>= 1;
        }
        (i, acc)
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
fn partition_point() {
    let data = [0, 2, 0, 1, 1, 0, 2, 0];
    let mut bit = BinaryIndexedTree::new(data.len(), 0, std::ops::Add::add);
    for (i, &x) in data.iter().enumerate() {
        bit.update(i, x);
    }
    assert!(data.iter().all(|&x| x >= 0));
    for i in 0..=data.iter().sum::<i32>() + 1 {
        let pred = |x| x < i;
        let (j, acc) = bit.partition_point(pred);
        let acc_naive = data.iter().take(j).sum::<i32>();
        assert_eq!(acc, acc_naive);
        assert!(j == data.len() || !pred(bit.prefix(j + 1)));
    }
}
