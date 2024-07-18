impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut max_sum = 0i64;
        let mut changed = 0;
        let mut min_change = i32::MAX;
        for n in nums {
            let n_xor_k = n ^ k;
            min_change = min_change.min((n-n_xor_k).abs());
            if n_xor_k > n {
                max_sum += i64::from(n_xor_k);
                changed += 1;
            } else {
                max_sum += i64::from(n);
            }
        }
        if changed % 2 == 0 {
            max_sum
        } else {
            max_sum - i64::from(min_change)
        }
        
    }
}
