impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 1 {
            return 1;
        }
        if l == 2 {
            return 0;
        }
        let mut even_sum = vec![0; l];
        let mut odd_sum = vec![0; l];
        even_sum[0] = nums[0];
        odd_sum[1] = nums[1];
        for i in 1..l {
            even_sum[i] = even_sum[i-1];
            odd_sum[i] = odd_sum[i-1];
            if i % 2 == 0 {
                even_sum[i] += nums[i];
            } else {
                odd_sum[i] += nums[i];
            }
        }
        let mut esum = 0;
        let mut osum = 0;
        let mut ans = 0;
        for i in (0..l).rev() {
            if i % 2 == 0 {
                let lhs = even_sum[i] - nums[i] + osum;
                let rhs = odd_sum[i] + esum;
                if lhs == rhs {
                    ans += 1;
                }
                esum += nums[i];
            } else {
                let lhs = odd_sum[i] - nums[i] + esum;
                let rhs = even_sum[i] + osum;
                if lhs == rhs {
                    ans += 1;
                }
                osum += nums[i];
            }
        }

        ans
    }
}

