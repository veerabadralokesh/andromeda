impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let (mut minsum, mut maxsum, mut cur_min, mut cur_max, mut total) = (i32::MAX, i32::MIN, 0, 0, 0);
        for &n in nums.iter() {
            total += n;
            cur_min = n.min(cur_min + n);
            cur_max = n.max(cur_max + n);
            minsum = minsum.min(cur_min);
            maxsum = maxsum.max(cur_max);
        }
        // println!("{maxsum} {minsum}");
        if maxsum < 0 {
            maxsum
        } else {
            maxsum.max(total - minsum)
        }
    }
}

