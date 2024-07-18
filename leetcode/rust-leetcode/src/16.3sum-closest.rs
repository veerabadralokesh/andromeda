
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        // println!("{:?}",nums);
        let n = nums.len();
        let mut ans = i32::MAX;
        let mut sum = 0;
        for i1 in 0..n {
            let n1 = nums[i1];
            let mut i2 = i1 + 1;
            let mut i3 = n-1;
            while i2 < i3 {
                sum = n1 + nums[i2] + nums[i3];
                // println!("{:?}", sum);
                if sum == target {
                    return sum;
                }
                if (sum-target).abs() < (ans-target).abs() {
                    ans = sum;
                }
                if sum < target {
                    i2 += 1;
                } else {
                    i3 -= 1;
                }
            }
        }
        ans
    }
}

