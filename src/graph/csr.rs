//! Compressed sparse row

struct Csr<T> {
    values: Box<[(usize, T)]>,
    pref: Box<[usize]>,
}

impl<T> std::ops::Index<usize> for Csr<T> {
    type Output = [(usize, T)];
    fn index(&self, i: usize) -> &Self::Output {
        &self.values[self.pref[i]..self.pref[i + 1]]
    }
}

impl<T: Copy + Default> Csr<T> {
    fn from_directed_edges(n: usize, edges: Vec<(usize, usize, T)>) -> Self {
        let mut values = vec![(0, T::default()); edges.len()].into_boxed_slice();
        let mut pref = vec![0; n + 1].into_boxed_slice();
        for &(u, _, _) in edges.iter() {
            pref[u] += 1;
        }
        for i in 0..n {
            pref[i + 1] += pref[i];
        }
        for (u, v, w) in edges {
            pref[u] -= 1;
            values[pref[u]] = (v, w);
        }
        Self { values, pref }
    }
    fn from_undirected_edges(n: usize, edges: Vec<(usize, usize, T)>) -> Self {
        let mut values = vec![(0, T::default()); edges.len() * 2].into_boxed_slice();
        let mut pref = vec![0; n + 1].into_boxed_slice();
        for &(u, v, _) in edges.iter() {
            pref[u] += 1;
            pref[v] += 1;
        }
        for i in 0..n {
            pref[i + 1] += pref[i];
        }
        for (u, v, w) in edges {
            pref[u] -= 1;
            values[pref[u]] = (v, w);
            pref[v] -= 1;
            values[pref[v]] = (u, w);
        }
        Self { values, pref }
    }
}

#[test]
fn from_directed_edges() {
    let edges = vec![(0, 2, -1), (2, 4, -2), (4, 0, -3), (0, 3, -4), (3, 3, -5)];
    let csr = Csr::from_directed_edges(5, edges.clone());
    assert_eq!(csr[0], [(3, -4), (2, -1)]);
    assert_eq!(csr[1], []);
    assert_eq!(csr[2], [(4, -2)]);
    assert_eq!(csr[3], [(3, -5)]);
    assert_eq!(csr[4], [(0, -3)]);
}

#[test]
fn from_undirected_edges() {
    let edges = vec![(0, 2, -1), (2, 4, -2), (4, 0, -3), (0, 3, -4), (3, 3, -5)];
    let csr = Csr::from_undirected_edges(5, edges.clone());
    assert_eq!(csr[0], [(3, -4), (4, -3), (2, -1)]);
    assert_eq!(csr[1], []);
    assert_eq!(csr[2], [(4, -2), (0, -1)]);
    assert_eq!(csr[3], [(3, -5), (3, -5), (0, -4)]);
    assert_eq!(csr[4], [(0, -3), (2, -2)]);
}

#[test]
fn unit_weight() {
    let edges = vec![(0, 0, ())];
    let csr = Csr::from_directed_edges(1, edges);
    let edge = csr[0][0];
    assert_eq!(std::mem::size_of_val(&edge), std::mem::size_of::<usize>());
}
