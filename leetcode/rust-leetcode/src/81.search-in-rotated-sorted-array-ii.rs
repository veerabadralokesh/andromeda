impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let (mut k, mut lo, mut hi) = (0usize, 0usize, n - 1);
        while lo < nums.len() && nums[lo] > target {
            lo += 1;
        }
        while hi > 0 && nums[hi] < target {
            hi -= 1;
        }
        while lo <= hi {
            k = (lo + hi)/2;
            if nums[k] == target {
                return true;
            }
            if nums[lo] <= nums[k] {
                if nums[lo] <= target && target <= nums[k] {
                    hi = k - 1;
                } else {
                    lo = k + 1;
                }
            } else {
                if nums[hi] >= target && nums[k] <= target {
                    lo = k + 1;
                } else {
                    hi = k - 1;
                }
            }
        }
        // println!("{lo} {hi}");
        // if nums[] == target {
        //     return true;
        // }
        false
    }
}

