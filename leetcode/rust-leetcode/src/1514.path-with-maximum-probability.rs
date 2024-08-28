use std::cmp::Ordering;
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let (mut u, mut v);
        for (e, prob) in edges.iter().zip(succ_prob) {
            (u, v) = (e[0] as usize, e[1] as usize);
            graph[u].push((v, prob));
            graph[v].push((u, prob));
        }
        let mut heap:BinaryHeap<(MinNonNan, usize)> = BinaryHeap::new();
        let (start, end) = (start_node as usize, end_node as usize);
        heap.push((MinNonNan(-1.0), start));
        let mut visited = vec![false; n];
        while let Some((MinNonNan(prob), u)) = heap.pop() {
            if u == end {
                return -prob;
            }
            visited[u] = true;
            for &(v, proba) in graph[u].iter() {
                if !visited[v] {
                    heap.push((MinNonNan(prob * proba), v));
                }
            }
        }
        0.0
    }
}

#[derive(PartialEq)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


