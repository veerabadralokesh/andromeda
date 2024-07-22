impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_sum = nums.iter().filter(|&n| *n & 1 == 0).sum::<i32>();
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let (val, idx) = (q[0], q[1] as usize);
            if nums[idx] & 1 == 0 {
                even_sum -= nums[idx];
            }
            nums[idx] += val;
            if nums[idx] & 1 == 0 {
                even_sum += nums[idx];
            }
            ans.push(even_sum);
        }
        ans
    }
}

