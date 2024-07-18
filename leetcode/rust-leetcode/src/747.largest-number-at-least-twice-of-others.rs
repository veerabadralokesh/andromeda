impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut mx, mut mx2) = (nums[0], i32::MIN);
        let mut mxidx = 0;
        for i in 1..nums.len() {
            if nums[i] > mx {
                mx2 = mx;
                mx = nums[i];
                mxidx = i;
            } else if nums[i] > mx2 {
                mx2 = nums[i];
            }
        }
        if mx >= 2 * mx2 {mxidx as _} else {-1}
    }
}

