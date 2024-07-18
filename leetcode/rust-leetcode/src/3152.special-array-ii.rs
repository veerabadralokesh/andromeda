impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut parity = vec![0; nums.len()];
        for i in 1..nums.len() {
            parity[i] += parity[i-1];
            if nums[i] & 1 == nums[i-1] & 1 {
                parity[i] += 1;
            }
        }
        queries.iter().map(|q| parity[q[0] as usize]==parity[q[1] as usize]).collect()
    }
}
