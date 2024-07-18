impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                if nums[i].len() + nums[j].len() == target.len() {
                    if target[..nums[i].len()] == nums[i] && target[nums[i].len()..] == nums[j] {
                        ans += 1;
                    }
                    if target[..nums[j].len()] == nums[j] && target[nums[j].len()..] == nums[i] {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

