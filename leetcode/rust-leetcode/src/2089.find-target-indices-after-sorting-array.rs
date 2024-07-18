impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut count, mut less_count) = (0, 0);
        for n in nums {
            if n < target {
                less_count += 1;
            } else if n == target {
                count += 1;
            }
        }
        (less_count..less_count+count).collect()
    }
}

