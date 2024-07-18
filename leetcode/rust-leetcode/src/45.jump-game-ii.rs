impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut jumps = vec![l as i32; l];
        jumps[0] = 0;
        for i in 0..l-1 {
            for j in 1..((nums[i]+1) as usize) {
                if i+j > l-1 {break;}
                jumps[i+j] = std::cmp::min(jumps[i+j], jumps[i] + 1);
            }
        }
        jumps[nums.len()-1]
    }
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut next = 0;
        let mut jumps = 0;
        let mut curr = 0;
        for i in 0..n-1 {
            next = i32::max(next-1, nums[i]);
            if curr == 0 {
                curr = next;
                next = 0;
                jumps += 1;
            }
            curr -= 1;
        }
        jumps
    }
}
