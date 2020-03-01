/// Maximum bipartite matching
/// O(VE)
///
/// # Examples
///
/// ```
/// use algorithms::graph::BipartiteMatching;
/// let mut bp = BipartiteMatching::new(3, 4);
/// bp.add_edge(0, 1);
/// bp.add_edge(0, 2);
/// bp.add_edge(1, 2);
/// bp.add_edge(2, 0);
/// let matching = bp.compute();
/// assert_eq!(1, matching[&0]);
/// assert_eq!(2, matching[&1]);
/// assert_eq!(0, matching[&2]);
/// ```
pub struct BipartiteMatching {
    left_num: usize,        // |V_left|
    right_num: usize,       // |V_right|
    graph: Vec<Vec<usize>>, // graph[left_idx] stores a list of right_idx
}

impl BipartiteMatching {
    pub fn new(left_num: usize, right_num: usize) -> Self {
        BipartiteMatching {
            left_num: left_num,
            right_num: right_num,
            graph: vec![Vec::new(); left_num],
        }
    }

    pub fn add_edge(&mut self, left_idx: usize, right_idx: usize) {
        assert!(left_idx < self.left_num);
        assert!(right_idx < self.right_num);
        self.graph[left_idx].push(right_idx);
    }

    pub fn compute(&self) -> ::std::collections::HashMap<usize, usize> {
        let mut matching = vec![usize::max_value(); self.left_num];
        let mut inv = vec![usize::max_value(); self.right_num];
        let mut visited = vec![false; self.left_num];

        let mut updated = true;
        while updated {
            updated = false;
            for x in visited.iter_mut() {
                *x = false;
            }

            for i in 0..self.left_num {
                if matching[i] == usize::max_value() {
                    if self.dfs(i, &mut visited, &mut matching, &mut inv) {
                        updated = true;
                    }
                }
            }
        }

        let mut result = ::std::collections::HashMap::new();
        for i in 0..self.left_num {
            if matching[i] != usize::max_value() {
                result.insert(i, matching[i]);
            }
        }
        result
    }

    fn dfs(
        &self,
        left_idx: usize,
        visited: &mut [bool], // visited[left_idx] is true if dfs is called for the vertex once
        matching: &mut [usize], // matching[left_idx] stores index of matching right vertex.
        inv: &mut [usize],    // inv[right_idx] stores index of matching left vertex.
    ) -> bool {
        if visited[left_idx] {
            return false;
        }
        visited[left_idx] = true;

        for &right_idx in &self.graph[left_idx] {
            if inv[right_idx] == usize::max_value() {
                matching[left_idx] = right_idx;
                inv[right_idx] = left_idx;
                return true;
            }
        }
        for &right_idx in &self.graph[left_idx] {
            if self.dfs(inv[right_idx], visited, matching, inv) {
                matching[left_idx] = right_idx;
                inv[right_idx] = left_idx;
                return true;
            }
        }
        false
    }
}
