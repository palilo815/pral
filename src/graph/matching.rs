//! Bipartite Matching (Kuhn's Algorithm)

struct BipartiteMatching {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
}

impl BipartiteMatching {
    fn new(n: usize, m: usize) -> Self {
        Self { n, m, adj: vec![vec![]; n] }
    }
    fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n && v < self.m);
        self.adj[u].push(v);
    }
    fn maximum_matching(&self) -> Vec<usize> {
        let mut check = vec![false; self.n];
        let mut wife = vec![usize::MAX; self.n];
        let mut hubby = vec![usize::MAX; self.m];
        while (0..self.n).any(|i| wife[i] == usize::MAX && self._dfs(i, &mut check, &mut wife, &mut hubby)) {
            check.fill(false);
        }
        wife
    }
    fn _dfs(&self, u: usize, check: &mut [bool], wife: &mut [usize], hubby: &mut [usize]) -> bool {
        check[u] = true;
        for &v in self.adj[u].iter() {
            if hubby[v] == usize::MAX || (!check[hubby[v]] && self._dfs(hubby[v], check, wife, hubby)) {
                wife[u] = v;
                hubby[v] = u;
                return true;
            }
        }
        false
    }
}
