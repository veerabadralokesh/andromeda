use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map:HashMap<usize, Vec<i32>> = HashMap::new();
        for (i, n) in group_sizes.into_iter().enumerate() {
            // println!("{i}, {n}");
            map.entry(n as usize).or_insert(Vec::new()).push(i as i32);
        }
        // println!("{:?}", map);
        let mut ans:Vec<Vec<i32>> = Vec::with_capacity(map.len());
        for (k, v) in map.into_iter() {
            for i in (0..v.len()).step_by(k) {
                // println!("{} {} {:?}", i, k, v[i..i+k].to_vec());
                ans.push(v[i..i+k].to_vec());
            }
        }
        ans
    }
}
