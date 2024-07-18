use std::collections::HashMap;
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0]
        }
        let n = nums.len();
        let mut nums = nums;
        nums.insert(0, 1);
        nums.push(1);
        let mut memory:HashMap<(usize,usize), i32> = HashMap::new();
        fn dp(map: &mut HashMap<(usize,usize), i32>, nums: &Vec<i32>, i:usize, j:usize) -> i32 {
            if i > j {return 0;}
            if map.contains_key(&(i,j)) {return *map.get(&(i,j)).unwrap()}
            let mut ans = 0;
            for k in i..j+1 {
                let coins = nums[k]*nums[i-1]*nums[j+1] + dp(map, nums, i, k-1)
                            + dp(map, nums, k+1, j);
                ans = ans.max(coins);
            }
            map.insert((i, j), ans);
            ans
        }
        dp(&mut memory, &nums, 1, n)
    }
}

/* */

impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        nums.insert(0, 1);
        nums.push(1);
        let n = nums.len();
        let mut dp = vec![vec![0i32; n]; n];
        for i in (1..n-1).rev() {
            for j in i..n-1 {
                for k in i..=j {
                    let coins = nums[i-1]*nums[k]*nums[j+1] + dp[i][k-1] + dp[k+1][j];
                    dp[i][j] = dp[i][j].max(coins);
                }
            }
        }
        dp[1][n-2]
    }
}
