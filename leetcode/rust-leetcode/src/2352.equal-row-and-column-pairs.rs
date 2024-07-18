use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut map:HashMap<Vec<i32>, i32> = HashMap::with_capacity(n);
        for i in 0..n {
            *map.entry(grid[i].clone()).or_insert(0) += 1;
        }
        let mut ans:i32 = 0;
        for j in 0..n {
            let mut cr:Vec<i32> = Vec::with_capacity(n);
            for i in 0..n {
                cr.push(grid[i][j]);
            }
            if map.contains_key(&cr) {
                ans += map.get(&cr).unwrap();
            }
        }
        ans
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut map:HashMap<String, i32> = HashMap::with_capacity(n);
        for i in 0..n {
            let mut sr = String::new();
            for j in 0..n {
                sr.push_str(grid[i][j].to_string().as_str());
                sr.push(',');
            }
            *map.entry(sr).or_insert(0) += 1;
        }
        let mut ans:i32 = 0;
        for j in 0..n {
            let mut sr = String::new();
            for i in 0..n {
                sr.push_str(grid[i][j].to_string().as_str());
                sr.push(',');
            }
            if map.contains_key(&sr) {
                ans += map.get(&sr).unwrap();
            }
        }
        ans
    }
}
