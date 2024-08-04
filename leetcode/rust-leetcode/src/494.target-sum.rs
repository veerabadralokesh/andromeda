use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        fn dfs(nums: &Vec<i32>, sum: i32, i: usize, target: &i32, memo: &mut HashMap<(i32, usize), i32>) -> i32 {
            if i == nums.len() {
                return (sum == *target) as i32;
            }
            match memo.get(&(sum, i)) {
                Some(&ways) => ways,
                None => {
                    let ways = dfs(nums, sum + nums[i], i+1, target, memo) + dfs(nums, sum - nums[i], i+1, target, memo);
                    memo.insert((sum, i), ways);
                    ways
                }
            }
        }
        let mut memo = HashMap::new();
        dfs(&nums, 0, 0, &target, &mut memo)
    }
}

