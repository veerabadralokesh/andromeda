impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut count, mut pivot) = (0, -1);
        for i in 0..n-1 {
            if nums[i] > nums[i+1] {
                count += 1;
                pivot = i as i32;
            }
        }
        if count == 0 {
            return 0;
        } else if count > 1 || nums[n - 1] > nums[0] {
            return -1;
        }
        (n - 1) as i32 - pivot
    }
}

