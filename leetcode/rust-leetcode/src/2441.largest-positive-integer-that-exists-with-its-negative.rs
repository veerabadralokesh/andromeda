impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut t = 0i32;
        while l < r {
            t = -1 * nums[l];
            if t == nums[r] {
                return nums[r];
            } else if t > nums[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        -1
    }
}

/* */

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut ps = [false; 1001];
        let mut ns = [false; 1001];
        let mut ans = -1;
        for num in nums {
            let n = num.abs() as usize;
            if num > 0 {
                ps[n] = true;
                if ns[n] {
                    ans = ans.max(num);
                }
            } else {
                ns[n] = true;
                if ps[n] {
                    ans = ans.max(-num);
                }
            }
        }
        ans
    }
}
