impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let (mut sum, mut count) = (0, 0);
        for &n in nums.iter() {
            if n % 6 == 0 {
                sum += n;
                count += 1;
            }
        }
        if count == 0 {0} else {sum/count}
    }
}

