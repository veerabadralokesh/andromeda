impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right, mut sum, n, mut ans) = (0, 0, 0, nums.len(), nums.len()+1);
        while right < n {
            while right < n && sum < target {
                sum += nums[right];
                right += 1;
            }
            while sum >= target {
                sum -= nums[left];
                ans = ans.min(right-left);
                left += 1;
            }
        }
        if ans == n+1 {ans = 0;}
        ans as i32
    }
}

/*
*/

use std::collections::VecDeque;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let total_sum = nums.clone().into_iter().sum::<i32>();
        let l = nums.len();
        if total_sum < target {
            return 0;
        }
        if total_sum == target {
            return l as i32;
        }
        let mut queue = VecDeque::new();
        for i in 0..l {
            if nums[i] >= target {
                return 1;
            } else if nums[i] < target {
                queue.push_back(((i, i), nums[i]));
            }
        }
        for i in 0..l-1 {
            let sum = nums[i] + nums[i+1];
            if sum >= target {
                return 2;
            } else if sum < target {
                queue.push_back(((i, i+1), sum));
            }
        }
        // println!("{:?}", queue);
        while queue.len() > 0 {
            let entry = queue.pop_front().unwrap();
            let i = entry.0.0;
            if i == 0 {continue;}
            let j = entry.0.1;
            if j == l-1 {continue;}
            let mut sum = entry.1;
            sum += (nums[i-1] + nums[j+1]);
            if sum >= target {
                return (j - i + 3) as i32;
            } else if sum < target {
                queue.push_back(((i-1, j+1), sum));
            }
        }
        0
    }
}
