use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map:HashMap<i32,(i32, (usize, usize))> = HashMap::new();
        let (mut max_n, mut max_count) = (vec![nums[0]], 1);
        for (i, &n) in nums.iter().enumerate() {
            match map.get_mut(&n) {
                Some(entry) => {
                    entry.0 += 1;
                    entry.1.1 = i;
                    if entry.0 > max_count {
                        max_n = vec![n];
                        max_count = entry.0;
                    } else if entry.0 == max_count {
                        max_n.push(n);
                    }
                },
                None => {
                    map.insert(n, (1, (i, i)));
                }
            }
        }
        let mut min_arr = i32::MAX;
        for n in max_n.iter() {
            match map.get(n) {
                None => unreachable!(),
                Some(entry) => {
                    min_arr = min_arr.min((entry.1.1 - entry.1.0 + 1) as i32);
                }
            }
        }
        min_arr
    }
}

