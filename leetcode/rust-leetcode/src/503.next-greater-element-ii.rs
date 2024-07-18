impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut nums2 = nums.to_vec();
        nums2.append(&mut nums.to_vec());
        let n = nums.len();
        let mut ans = vec![-1;nums.len()];
        for i in 0..2*n {
            while !stack.is_empty() && nums[*stack.last().unwrap() % n] < nums2[i] {
                let idx = stack.pop().unwrap();
                if idx >= nums.len() {
                    continue;
                }
                ans[idx] = nums2[i];
            }
            stack.push(i);
        }
        ans
    }
}
