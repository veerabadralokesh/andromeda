impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let (mut flipped, mut ans) = (0, 0);
        for i in 0..nums.len() {
            if i > 2 && nums[i - 3] == 2 {
                flipped -= 1;
            }
            if flipped & 1 == nums[i] {
                if i + 3 > nums.len() {
                    return -1;
                }
                nums[i] = 2;
                flipped += 1;
                ans += 1;
            }
        }
        ans
    }
}

