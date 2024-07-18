impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut pivots = Vec::new();
        let l = nums.len();
        for n in nums {
            match(n.cmp(&pivot)) {
                std::cmp::Ordering::Less => {
                    left.push(n);
                },
                std::cmp::Ordering::Equal => {
                    pivots.push(n);
                },
                std::cmp::Ordering::Greater => {
                    right.push(n);
                }
            }
        }
        let mut nums = Vec::with_capacity(l);
        nums.extend(left);
        nums.extend(pivots);
        nums.extend(right);
        nums
    }
}

/* */

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        for &i in &nums {
            if i < pivot {
                ans.push(i);
            }
        }
        for &i in &nums {
            if i == pivot {
                ans.push(i);
            }
        }
        for &i in &nums {
            if i > pivot {
                ans.push(i);
            }
        }
        ans
    }
}
