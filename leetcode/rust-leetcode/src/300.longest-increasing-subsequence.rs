impl Solution {
    pub fn length_of_lis(mut nums: Vec<i32>) -> i32 {
        let mut dp = vec![1i32; nums.len()];

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j]+1);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}

/* */

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut v: Vec<i32> = vec![*nums.first().unwrap()];
        for n in nums.iter() {
            if n > v.last().unwrap() {
                v.push(*n);
            } else {
                let pos = v.binary_search(n);
                if let Err(pos) = pos {
                    if v[pos] > *n {
                        v[pos] = *n;
                    }
                }
            }
        }
        v.len() as i32
    }
}
