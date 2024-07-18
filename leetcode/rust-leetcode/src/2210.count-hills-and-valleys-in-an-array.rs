impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let (mut i, mut ans) = (0, 0);
        for j in 1..nums.len()-1 {
            if nums[j] > nums[i] {
                if nums[j] > nums[j+1] {
                    ans += 1;
                    i = j;
                } else if nums[j] < nums[j+1] {
                    i = j;
                }
            } else if nums[j] < nums[i] {
                if nums[j] < nums[j+1] {
                    ans += 1;
                    i = j;
                } else if nums[j] > nums[j+1] {
                    i = j;
                }
            }
        }
        ans
    }
}

