use std::collections::HashMap;
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let l = nums.len();
        for i in 0..l-3 {
            for j in i+1..l-2 {
                let mut map = HashMap::new();
                for k in j+1..l {
                    ans += *map.get(&nums[k]).unwrap_or(&0);
                    *map.entry((nums[i]+nums[j]+nums[k])).or_insert(0) += 1;
                }
            }
        }
        ans
    }
}

/* */

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let l = nums.len();
        for a in 0..l-3 {
            for b in a+1..l-2 {
                for c in b+1..l-1 {
                    for d in c+1..l {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}


