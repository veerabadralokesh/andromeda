impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut i = 0;
        while i < l-1 {
            if nums[i] != nums[i+1] {
                return nums[i];
            }
            i += 2;
        }
        nums[l-1]
    }
}

/* */

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            match l + (r - l) / 2 {
                mid if nums[mid] == nums[mid ^ 1] => l = mid + 1,
                mid => r = mid,
            }
        }
        nums[l]
    }
}
