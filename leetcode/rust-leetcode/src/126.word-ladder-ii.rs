use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut position_bytes = vec![HashSet::with_capacity(26); begin_word.len()];
        let dict = word_list.into_iter().map(|w| {
            let wb = w.into_bytes();
            for (i, &b) in wb.iter().enumerate() {
                position_bytes[i].insert(b);
            }
            wb
        }).collect::<HashSet<_>>();
        let ew = end_word.into_bytes();
        if !dict.contains(&ew) {
            return vec![];
        }
        let mut bw = begin_word.into_bytes();
        let bfs = || -> (usize, HashMap<Vec<u8>, HashSet<Vec<u8>>>) {
            let mut temp = b'a';
            let mut transformations = 1;
            let mut q = VecDeque::with_capacity(dict.len());
            q.push_back(bw.to_vec());
            let mut visited = HashSet::with_capacity(dict.len());
            let mut graph = HashMap::new();
            let mut found = false;
            while !q.is_empty() {
                for _ in 0..q.len() {
                    if let Some(mut wb) = q.pop_front() {
                        if visited.contains(&wb) {
                            continue;
                        }
                        visited.insert(wb.to_vec());
                        let parent = wb.to_vec();
                        for i in 0..wb.len() {
                            temp = wb[i];
                            for &b in position_bytes[i].iter() {
                                if b != temp {
                                    wb[i] = b;
                                    if !visited.contains(&wb) && dict.contains(&wb) {
                                        graph.entry(wb.to_vec()).or_insert(HashSet::new()).insert(parent.to_vec());
                                        q.push_back(wb.to_vec());

                                        if wb == ew {
                                            found = true;
                                        }
                                    }
                                }
                            }
                            wb[i] = temp;
                        }
                    }
                }
                transformations += 1;
                if found {
                    return (transformations, graph);
                }
            }
            (0, graph)
        };
        let (transformations, graph) = bfs();
        let mut ans = vec![];
        if transformations == 0 {
            return ans;
        }
        let mut seq = Vec::with_capacity(transformations);

        fn dfs(ans: &mut Vec<Vec<String>>, seq: &mut Vec<Vec<u8>>, ew: &Vec<u8>, bw: &Vec<u8>, graph: &HashMap<Vec<u8>, HashSet<Vec<u8>>>, t: usize) {
            if t == seq.len() {
                return;
            }
            seq.push(ew.to_vec());
            if *ew == *bw {
                ans.push(
                    seq.iter().rev().cloned()
                    .map(|wb| wb.iter().map(|b| *b as char).collect::<String>()
                    ).collect::<Vec<_>>()
                );
            } else {
                match graph.get(ew) {
                    None => {},
                    Some(set) => {
                        for parent in set.iter() {
                            dfs(ans, seq, parent, bw, graph, t);
                        }
                    }
                }
            }
            seq.pop();
        }
        dfs(&mut ans, &mut seq, &ew, &bw, &graph, transformations);
        ans
    }
}

