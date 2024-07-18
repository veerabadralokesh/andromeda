impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let nums = nums.iter().map(|&n| n & 1).collect::<Vec<i32>>();
        let n = nums.len() as i32;
        let ones = nums.iter().filter(|&n| *n == 1).count() as i32;
        let zeros = n - ones;
        let mut alt = 1;
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if nums[i] & 1 != prev {
                alt += 1;
                prev = 1-prev;
            }
        }
        alt.max(ones.max(zeros))
    }
}

