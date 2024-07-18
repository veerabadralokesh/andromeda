impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len()-1 {
            for j in i+1..nums.len() {
                ans += (nums[i] ^ nums[j]).count_ones();
            }
        }
        ans as i32
    }
}

/* */

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let l = nums.len() as i32;
        
        for i in 0..32 {
            let mut count = 0;
            for &num in &nums {
                count += (num >> i) & 1;
            }
            ans += count * (l - count);
        }

        ans as i32
    }
}
