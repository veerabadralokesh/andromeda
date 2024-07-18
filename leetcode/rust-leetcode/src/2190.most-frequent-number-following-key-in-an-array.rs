impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut counts = [0; 1001];
        let mut max_count = 0;
        let mut ans = -1;
        for i in 0..nums.len()-1 {
            if nums[i] == key {
                let n = nums[i+1] as usize;
                counts[n] += 1;
                if counts[n] > max_count {
                    max_count = counts[n];
                    ans = nums[i+1];
                }
            }
        }
        ans
    }
}

