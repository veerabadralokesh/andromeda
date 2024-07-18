impl Solution {
    pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
        let mut maxor = 0i32;
        nums.sort();
        for i in 0..nums.len() {
            let n1 = nums[i];
            for j in i+1..nums.len() {
                let n2 = nums[j];
                if (n2-n1) <= n1 {
                    maxor = maxor.max(n1^n2);
                }
            }
        }
        maxor
    }
}
