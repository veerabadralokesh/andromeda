use std::collections::{HashMap,BinaryHeap};
use std::cmp::Reverse;
struct Graph {
    g: HashMap<i32, Vec<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {

    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = HashMap::with_capacity(edges.len());
        for e in edges {
            g.entry(e[0]).or_insert(Vec::new()).push((e[1], e[2]));
        }
        for i in 0..n {
            g.entry(i).or_insert(Vec::new());
        }
        Self {g}
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.g.entry(edge[0]).or_insert(Vec::new()).push((edge[1], edge[2]));
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut dist = vec![i32::MAX; self.g.len()];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), node1));
        while let Some((Reverse(d), u)) = heap.pop() {
            if u == node2 {
                return d;
            }
            match self.g.get(&u) {
                None => {},
                Some(vec) => {
                    for &(v, c) in vec.iter() {
                        if d + c < dist[v as usize] {
                            dist[v as usize] = d + c;
                            heap.push((Reverse(d + c), v));
                        }
                    }
                }
            }
        }
        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

