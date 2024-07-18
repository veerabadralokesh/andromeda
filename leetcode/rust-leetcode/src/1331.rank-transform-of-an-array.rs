impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        if arr.len() == 0 {
            return arr;
        }
        let mut sarr = arr.to_vec();
        sarr.sort();
        let mut rank = 1;
        let mut map = std::collections::HashMap::new();
        map.insert(sarr[0], 1);
        for i in 1..sarr.len() {
            if sarr[i] != sarr[i-1] {
                rank += 1;
                map.insert(sarr[i], rank);
            }
        }
        let mut ranks = vec![1;arr.len()];
        for (i, n) in arr.iter().enumerate() {
            ranks[i] = *map.get(n).unwrap();
        }
        ranks
    }
}

