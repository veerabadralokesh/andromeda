impl Solution {
    pub fn num_subarrays_with_sum(mut nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0i32;
        nums.insert(0, 0);
        for i in 1..nums.len() {
            if nums[i] == goal {
                ans += 1;
            }
            nums[i] += nums[i-1];
        }
        for i in 1..nums.len() {
            for j in i+1..nums.len() {
                if nums[j]-nums[i-1] == goal {
                    ans += 1;
                }
            }
        }
        ans
    }
}

/* */

use std::collections::HashMap;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut sum = 0;
        let mut counts = HashMap::new();
        let mut count = 0;
        counts.insert(0, 1);
        for num in nums {
            sum += num;
            count += *counts.entry(sum - goal).or_insert(0);
            *counts.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

/* */

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut sum = 0;
        let n = nums.len();
        let mut counts = vec![0; 2*n + 1];
        let mut count = 0;
        counts[n] = 1;
        for num in nums {
            sum += num;
            count += counts[n + (sum - goal) as usize];
            counts[n + (sum as usize)] += 1;
        }
        count
    }
}
