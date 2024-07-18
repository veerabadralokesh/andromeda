impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as i64;
        let mut nums = nums.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let sum = nums.iter().sum::<i64>();
        let reminder = sum % p;
        if reminder == 0 {
            return 0;
        }
        let mut map = std::collections::HashMap::new();
        let l = nums.len() as i64;
        let mut ans = l;
        let mut prefix = 0;
        let mut target = 0;
        map.insert(0, -1);
        for (i, &n) in nums.iter().enumerate() {
            prefix += n;
            prefix %= p;
            target = (prefix - reminder + p) % p;
            if map.contains_key(&target) {
                ans = ans.min(i as i64 - map.get(&target).unwrap())
            }
            map.insert(prefix, i as i64);
        }
        if ans == l {-1} else {ans as _}
    }
}

