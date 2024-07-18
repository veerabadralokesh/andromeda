impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut mc:i32 = 0;
        let mut c:i32 = 0;
        for n in nums.iter() {
            if *n == 1 {
                c += 1;
            } else {
                c = 0;
            }
            if c > mc {
                mc = c;
            }
        }
        mc
    }
}

impl Solution2 {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut curr_len = 0;
        let mut max_len = 0;
        
        for num in nums{
            if num == 1{
                curr_len += 1;
                max_len = curr_len.max(max_len);
            }
            else{
                curr_len = 0;
            }
        }
        return max_len
        
    }
}