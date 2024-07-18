impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (mut mx, mut mn) = (i32::MIN, i32::MAX);
        for &n in nums.iter() {
            mx = mx.max(n);
            mn = mn.min(n);
        }
        (mx - mn - 2 * k).max(0)
    }
}

