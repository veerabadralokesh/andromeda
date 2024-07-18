use std::collections::{HashSet,HashMap,VecDeque};
impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        let n = is_connected.len();
        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    graph.entry(i).or_insert(HashSet::new()).insert(j);
                    graph.entry(j).or_insert(HashSet::new()).insert(i);
                }
            }
        }
        let mut city_province = vec![0; n];
        fn bfs(graph: &HashMap<usize,HashSet<usize>>, province: i32, city: usize, city_province: &mut Vec<i32>) {
            let mut q = VecDeque::new();
            q.push_back(city);
            let mut set = HashSet::new();
            set.insert(city);
            city_province[city] = province;
            while let Some(x) = q.pop_front() {
                match(graph.get(&x)) {
                    Some(hs) => {
                        for &nc in hs.into_iter() {
                            if !set.contains(&nc) {
                                set.insert(nc);
                                city_province[nc] = province;
                                q.push_back(nc);
                            }
                        }
                    },
                    None => {}
                }
            }
        }

        let mut provinces = 0;

        for c in 0..n {
            if city_province[c] == 0 {
                provinces += 1;
                bfs(&graph, provinces, c, &mut city_province);
            }
        }

        provinces
    }
}
