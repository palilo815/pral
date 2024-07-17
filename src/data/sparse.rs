//! Segment data

struct SparseTable<T, F> {
    size: usize,
    data: Box<[Box<[T]>]>,
    e: T,
    f: F,
}

impl<T, F> SparseTable<T, F>
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    fn new(data: Vec<T>, e: T, f: F) -> Self {
        let size = data.len();
        let data = std::iter::successors(Some((data, 1)), |(prev, k)| {
            let curr = prev.windows(k + 1).map(|w| f(w[0], w[*k])).collect::<Vec<_>>();
            if curr.is_empty() {
                None
            } else {
                Some((curr, k << 1))
            }
        })
        .map(|x| x.0.into_boxed_slice())
        .collect();
        Self { size, data, e, f }
    }
    fn prod(&self, range: std::ops::Range<usize>) -> T {
        assert!(range.start <= range.end && range.end <= self.size);
        if range.is_empty() {
            return self.e;
        }
        let k = range.len().ilog2() as usize;
        (self.f)(self.data[k][range.start], self.data[k][range.end - (1 << k)])
    }
}

#[test]
fn test() {
    let a = vec![-1, 3, 0, 4, 9, 2, 2, 8];
    let sparse = SparseTable::new(a.clone(), i32::MAX, std::cmp::min);
    for j in 0..a.len() {
        for i in 0..j {
            assert_eq!(*a[i..j].iter().min().unwrap(), sparse.prod(i..j));
        }
    }
    let a = (0..3_usize).cycle().take(9).collect::<Vec<_>>();
    let sparse = SparseTable::new(a.clone(), usize::MIN, std::cmp::max);
    for j in 0..a.len() {
        for i in 0..j {
            assert_eq!(*a[i..j].iter().max().unwrap(), sparse.prod(i..j));
        }
    }
}
