impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min_sum = usize::MAX;
        let mut ans = Vec::new();
        let mut map = std::collections::HashMap::new();
        for (i, s) in list1.into_iter().enumerate() {
            map.insert(s, i);
        }
        let mut map2 = std::collections::HashMap::new();
        for (i, s) in list2.into_iter().enumerate() {
            match map.get(&s) {
                None => {},
                Some(idx) => {
                    if i + idx < min_sum {
                        min_sum = i + idx;
                    }
                    map2.insert(s, i + idx);
                }
            }
        }
        for (s, c) in map2.into_iter() {
            if c == min_sum {
                ans.push(s.clone());
            }
        }
        ans
    }
}

