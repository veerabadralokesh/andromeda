impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut inc, mut dec, mut len) = (0, 0, 1);
        for i in 1..nums.len() {
            if nums[i] > nums[i-1] {
                len += 1;
            } else {
                len = 1;
            }
            inc = inc.max(len);
        }
        inc = inc.max(len);
        len = 1;
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                len += 1;
            } else {
                len = 1;
            }
            dec = dec.max(len);
        }
        dec = dec.max(len);
        inc.max(dec)
    }
}

