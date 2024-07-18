use std::collections::HashMap;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        for item in items1.iter() {
            *map.entry(item[0]).or_insert(0) += item[1];
        }
        for item in items2.iter() {
            *map.entry(item[0]).or_insert(0) += item[1];
        }
        let mut ret = Vec::new();
        for (value, weight) in map.into_iter() {
            ret.push([value, weight].to_vec());
        }
        ret.sort();
        ret
    }
}

