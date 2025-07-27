//! Disjoint Set Union _(Union Find)_
//!
//! - **Author** &emsp; palilo
//! - **Source** &emsp; well known
//! - **Update** &emsp; 2023-09-01
//!
//! A DSU is a data structure that keeps track of a set of elements partitioned into a number of disjoint (non-overlapping) subsets.
//!
//! # Key Operations
//!
//! 1. `find`: Determines which subset an element belongs to.
//! 2. `union`: Joins two subsets into a single subset.
//!
//! # Time Complexity
//!
//! The time complexity of DSU operations is nearly constant on average, specifically O(α(n)), where α(n) is the inverse of the Ackermann function.
//! The Ackermann function grows extremely fast, so its inverse, α(n), is very slow-growing.
//! For all practical purposes, α(n) can be considered a constant less than 5.
//!
//! This near-constant time complexity is achieved by using two optimizations:
//!
//!  * **Path Compression**: During a find operation, this optimization flattens the structure of the tree by making every node on the find path point directly to the root.
//!  * **Union by Size/Rank**: During a union operation, this optimization attaches the smaller tree to the root of the larger tree. This keeps the tree structure relatively flat.

/// A DSU with basic operations.
///
/// # Note
///
/// It is intended that `p` is public. It makes much easier to iterate the all roots or unite the nodes in different ways.
///
/// No [`assert!`], because the most important method [`Self::find`] is reculsively executed.
struct DisjointSet {
    p: Box<[i32]>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { p: vec![-1; n].into() }
    }
    fn find(&mut self, u: usize) -> usize {
        if self.p[u] < 0 {
            u
        } else {
            let root = self.find(self.p[u] as usize);
            self.p[u] = root as i32;
            root
        }
    }
    fn unite(&mut self, u: usize, v: usize) -> bool {
        let mut u = self.find(u);
        let mut v = self.find(v);
        if u == v {
            return false;
        }
        if self.p[u] > self.p[v] {
            std::mem::swap(&mut u, &mut v);
        }
        self.p[u] += self.p[v];
        self.p[v] = u as i32;
        true
    }
    fn clear(&mut self) {
        self.p.fill(-1);
    }
    fn same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }
    fn size_of(&mut self, u: usize) -> usize {
        let root = self.find(u);
        (-self.p[root]) as usize
    }
    fn num_components(&self) -> usize {
        self.p.iter().filter(|x| x.is_negative()).count()
    }
}

#[test]
fn test() {
    use super::dsu::DisjointSet;
    let n = 8;
    let mut dsu = DisjointSet::new(n);
    assert!(dsu.unite(0, 1));
    assert!(dsu.unite(1, 2));
    assert!(!dsu.unite(2, 0));
    assert!(dsu.same(2, 0));
    assert!(dsu.unite(6, 7));
    assert!(dsu.unite(4, 5));
    assert_eq!(dsu.size_of(0), 3);
    assert_eq!(dsu.size_of(1), 3);
    assert_eq!(dsu.size_of(2), 3);
    assert_eq!(dsu.size_of(3), 1);
    assert_eq!(dsu.num_components(), 4);
    dsu.clear();
    assert_eq!(dsu.num_components(), 8);
}
