impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut found_big = false;
        let n = nums.len() as i32;
        for i in 0..nums.len() {
            if nums[i] == n {
                found_big = true;
                ans += n - (i as i32) - 1;
            }
            if nums[i] == 1 {
                ans += (i as i32);
                if found_big {
                    ans -= 1;
                }
            }
        }
        ans
    }
}

