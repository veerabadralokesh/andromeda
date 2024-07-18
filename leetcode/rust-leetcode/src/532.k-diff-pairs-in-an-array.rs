use std::collections::HashSet;
impl Solution {
    pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut set = HashSet::new();
        let (mut left, mut right) = (0, 1);
        let mut ans = 0;
        while right < nums.len() {
            let diff = nums[right] - nums[left];
            if diff == k {
                set.insert((nums[left], nums[right]));
                right += 1;
                left += 1;
            } else if diff < k {
                right += 1;
            } else {
                left += 1;
            }
            if left == right {
                right += 1;
            }
        }
        set.len() as i32
    }
}

