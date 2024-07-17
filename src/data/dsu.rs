//! Disjoint set union

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
            let rt = self.find(self.p[u] as usize);
            self.p[u] = rt as i32;
            rt
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
