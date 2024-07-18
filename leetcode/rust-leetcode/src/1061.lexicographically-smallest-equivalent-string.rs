use std::collections::{HashSet,VecDeque};
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut nodes = vec![HashSet::new(); 26];
        for (i, j) in s1.clone().into_bytes().into_iter().zip(s2.clone().into_bytes()) {
            let i_idx = (i - b'a') as usize;
            let j_idx = (j - b'a') as usize;
            nodes[i_idx].insert(j_idx);
            nodes[j_idx].insert(i_idx);
        }
        let mut map = (0..26).map(|i| (i + b'a')).collect::<Vec<u8>>();
        for i in 0..26 {
            let value = (i + b'a');
            let mut visited = [false; 26];
            let iidx = i as usize;
            if map[iidx] <= value && nodes[iidx].len() > 0 {
                let mut q = VecDeque::new();
                for &b in nodes[iidx].iter() {
                    q.push_back(b);
                }
                while let Some(idx) = q.pop_front() {
                    visited[idx] = true;
                    map[idx] = map[idx].min(value);
                    for &nn in nodes[idx].iter() {
                        if !visited[nn] {
                            q.push_back(nn);
                        }
                    }
                }
            }
        }
        base_str.into_bytes().iter().map(|&b| map[(b - b'a') as usize] as char).collect()
    }
}

