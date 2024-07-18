impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let l = nums.len();
        nums[l-1]*nums[l-2] - nums[1]*nums[0]
    }
}

/* */

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        let (mut nl, mut nl1, mut n0, mut n1) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);
        for n in nums {
            if n > nl {
                nl1 = nl;
                nl = n;
            } else if n > nl1 {
                nl1 = n;
            }
            if n < n0 {
                n1 = n0;
                n0 = n;
            } else if n < n1 {
                n1 = n;
            }
        }
        nl * nl1 - n1 * n0
    }
}

