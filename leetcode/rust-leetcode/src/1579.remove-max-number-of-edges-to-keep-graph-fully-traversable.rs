use std::collections::*;
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        edges.sort_by_key(|e| -e[0]);
        let n = n as usize;
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut required = 0;

        for e in edges.iter() {
            let (etype, u, v) = (e[0], e[1] as usize, e[2] as usize);
            match etype {
                3 => {
                    if alice.unite(u, v) | bob.unite(u, v) {
                        required += 1;
                    }
                },
                2 => {
                    if bob.unite(u, v) {
                        required += 1;
                    }
                },
                _ => {
                    if alice.unite(u, v) {
                        required += 1;
                    }
                }
            }
        }
        if alice.n == 1 && bob.n == 1 {(edges.len() - required) as _} else {-1}
    }
}

#[derive(Debug)]
struct UnionFind {
    nodes: Vec<usize>,
    n: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (0..=n).collect::<Vec<_>>(),
            n
        }
    }

    fn unite(&mut self, u: usize, v: usize) -> bool {
        let uconnected = self.find(u);
        let vconnected = self.find(v);
        if uconnected != vconnected {
            self.nodes[uconnected] = v;
            self.n -= 1;
            return true;
        }
        false
    }

    fn find(&mut self, n: usize) -> usize {
        if self.nodes[n] != n {
            self.nodes[n] = self.find(self.nodes[n]);
        }
        self.nodes[n]
    }
}

