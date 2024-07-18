impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
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
        if nl1 != nl && nl1 != n0 {
            return nl1;
        }
        if n1 != nl && n1 != n0 {
            return n1;
        }
        -1
    }
}

/* */

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return -1;
        }
        let mut arr = [nums[0], nums[1], nums[2]];
        arr.sort();
        arr[1]
    }
}

