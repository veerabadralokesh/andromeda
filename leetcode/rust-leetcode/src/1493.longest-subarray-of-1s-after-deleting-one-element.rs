impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            if nums[right] == 0 {
                count += 1;
            }

            while count > 1 {
                if nums[left] == 0 {
                    count -= 1;
                }
                left += 1;
            }

            max = max.max(right - left);
            right += 1;
        }

        max as i32
    }
}

/* */

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut k : i32 = 1;
        for &n in &nums {
            if n == 0 {
                k -= 1;
            }
            if k < 0 {
                if nums[i] == 0 {
                    k += 1;
                }
                i += 1;
            }
        }
        return (nums.len() - i - 1) as i32;
    }
}

