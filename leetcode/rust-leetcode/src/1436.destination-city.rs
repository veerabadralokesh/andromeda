use std::collections::HashSet;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut starts = HashSet::new();
        let mut ends = HashSet::new();
        for path in paths {
            starts.insert(path[0].clone());
            ends.insert(path[1].clone());
        }
        ends.difference(&starts).last().unwrap().to_string()
    }
}

/* */

use std::collections::HashMap;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = HashMap::with_capacity(paths.len());
        let mut start = paths[0][0].clone();
        for path in paths.into_iter() {
            let key = path[0].clone();
            let value = path[1].clone();
            map.insert(key, value);
        }
        loop {
            match map.get(&start) {
                Some(next) => {start = next.to_string()},
                None => {
                    break;
                }
            }
        }
        start
    }
}
