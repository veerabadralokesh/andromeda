impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        if l == 1 {
            return 0;
        }
        let mut rpsum = nums.to_vec();
        for i in (0..l-1).rev() {
            rpsum[i] += rpsum[i+1];
        }
        if rpsum[1] == 0 {
            return 0;
        }
        let mut lpsum = nums.to_vec();
        for i in 0..nums.len() {
            if i == 0 {
                continue;
            }
            lpsum[i] += lpsum[i-1];
            if rpsum[i] == lpsum[i] {
                return i as i32;
            }
        }
        -1
    }
}

/* */

impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let sum:i32 = nums.iter().sum();
        let mut leftsum:i32 = 0;
        for (i, n) in nums.iter().enumerate() {
            if leftsum == sum - n - leftsum {
                return i as i32;
            }
            leftsum += n;
        }
        -1
    }
}

