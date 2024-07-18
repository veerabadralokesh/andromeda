impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        for i in 0..((k as usize)%l) {
            let n = nums.pop().unwrap();
            nums.insert(0, n);
        }
    }
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let uk: usize = k as usize % nums.len();
        let mut r: Vec<i32> = vec![];
        r.extend_from_slice(&nums[nums.len() - uk..]);
        r.extend_from_slice(&nums[..nums.len() - uk]);
        *nums = r;
    }
}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        
        nums.reverse();
        
        nums[..k].reverse();
        
        nums[k..].reverse();
    }
}
