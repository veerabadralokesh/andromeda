use std::collections::{HashMap,HashSet,VecDeque};
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let l = arr.len();
        if l < 3 {
            return l as i32 - 1;
        }
        let target = l - 1;
        let mut connections = HashMap::new();
        for i in (0..l).rev() {
            connections.entry(arr[i]).or_insert(Vec::new()).push(i);
        }
        for k in connections.clone().keys() {
            let indices = connections.get(k).unwrap();
            if indices.len() > 2 {
                let mut idxs = Vec::with_capacity(indices.len());
                idxs.push(indices[0]);
                for i in 1..indices.len()-1 {
                    if indices[i] == indices[i-1] - 1 && indices[i+1] == indices[i] - 1{
                        continue;
                    }
                    idxs.push(indices[i]);
                }
                idxs.push(indices[indices.len()-1]);
                connections.insert(*k, idxs);
            }
        }
        let mut jumps = 0;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(0);
        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(idx) = q.pop_front() {
                    if visited.contains(&idx) {
                        continue;
                    }
                    if idx == target {
                        return jumps;
                    }
                    visited.insert(idx);
                    for &i in connections.get(&arr[idx]).unwrap().iter() {
                        if i != idx && !visited.contains(&i) {
                            q.push_back(i);
                        }
                    }
                    if idx < target {
                        q.push_back(idx + 1);
                    }
                    if idx > 0 {
                        q.push_back(idx - 1);
                    }
                    connections.insert(arr[idx], Vec::new());
                }
            }
            jumps += 1;
        }
        l as i32 - 1
    }
}

