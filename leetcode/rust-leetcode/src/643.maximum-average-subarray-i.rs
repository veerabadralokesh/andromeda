impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut ans:f64 = -10e5;
        let mut temp:f64 = 0.0;
        let k = k as usize;
        temp = (nums[0..k].iter().sum::<i32>()) as f64;
        ans = temp;
        for i in k..nums.len() {
            temp += ((nums[i] - nums[i-k]) as f64);
            if temp > ans {
                ans = temp;
            }
        }
        ans / (k as f64)
    }
}

impl Solution2 {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut k = k as usize;
        let mut k_f = k as f64;
        let mut sum: i32 = nums[0..k].iter().sum();
        let mut max_avg = sum as f64 / k as f64;

        for i in k..nums.len() {
            sum -= nums[i-k];
            sum += nums[i];
            max_avg = max_avg.max(sum as f64 / k_f);
        }
        return max_avg;
    }
}
