use std::collections::{HashMap,HashSet,VecDeque};
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        if source == destination {
            return true;
        }
        let mut graph:HashMap<i32,HashSet<i32>> = HashMap::new();
        for edge in edges {
            let (s, d) = (edge[0],edge[1]);
            if s == source && d == destination {
                return true;
            }
            graph.entry(s).or_insert(HashSet::new()).insert(d);
            graph.entry(d).or_insert(HashSet::new()).insert(s);
        }
        let mut source_connected:HashSet<i32> = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(source);
        while let Some(s) = q.pop_front() {
            for &v in graph.get(&s).unwrap().into_iter() {
                if v == destination {
                    return true;
                }
                if !source_connected.contains(&v) && graph.contains_key(&v){
                    q.push_back(v);
                    source_connected.insert(v);
                }
            }
        }
        false
    }
}
