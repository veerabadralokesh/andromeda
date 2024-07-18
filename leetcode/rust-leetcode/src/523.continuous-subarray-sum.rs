impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let mut map = std::collections::HashMap::new();
        let k = k as i64;
        map.insert(nums[0]%k, 0);
        for i in 1..nums.len() {
            nums[i] = (nums[i]+nums[i-1]) % k;
            if nums[i] == 0 {
                return true;
            }
            if *map.get(&nums[i]).unwrap_or(&i) < i-1 {
                return true;
            }
            if !map.contains_key(&nums[i]) {
                map.insert(nums[i], i);
            }
        }
        false
    }
}

