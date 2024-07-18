impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut prefix = vec![0; l];
        let mut suffix = vec![0; l];
        let mut counts = [0; 51];
        let mut count = 0;
        for (i, &n) in nums.iter().enumerate() {
            let n = n as usize;
            if counts[n] == 0 {
                counts[n] = 1;
                count += 1;
            }
            prefix[i] = count;
        }
        count = 0;
        for (i, &n) in nums.iter().enumerate().rev() {
            let n = n as usize;
            if counts[n] == 1 {
                counts[n] = 0;
                count += 1;
            }
            suffix[i] = count;
            if i > 0 {
                prefix[i-1] -= suffix[i];
            }
        }
        // let mut ans = prefix.to_vec();
        // for i in 0..nums.len()-1 {
        //     ans[i] -= suffix[i+1];
        // }
        // ans
        prefix
    }
}

