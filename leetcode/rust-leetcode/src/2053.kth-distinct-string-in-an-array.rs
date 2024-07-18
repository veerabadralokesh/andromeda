impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut map = std::collections::HashMap::new();
        for s in arr.iter() {
            *map.entry(s).or_insert(0) += 1;
        }
        let mut counter = 0;
        for s in arr.iter() {
            match map.get(&s) {
                Some(1) => {
                    counter += 1;
                    if counter == k {
                        return s.to_string();
                    }
                },
                _ => {}
            }
        }
        String::new()
    }
}

