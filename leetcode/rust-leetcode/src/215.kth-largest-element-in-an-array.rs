use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(n);
        }
        for _ in 0..k-1 {
            heap.pop();
        }
        heap.pop().unwrap()
    }
}

/* */

// LEARN

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let hi = nums.len() - 1;
        Self::kth_largest(&mut nums, k as usize, 0, hi)
    }

    fn kth_largest(nums: &mut Vec<i32>, k: usize, lo: usize, hi: usize) -> i32 {
        let (l, r) = Self::partition(nums, lo, hi);

        if k >= l + 1 && k <= r + 1 {
            nums[l]
        } else if k <= l {
            Self::kth_largest(nums, k, lo, l - 1)
        } else {
            Self::kth_largest(nums, k, r + 1, hi)
        }
    }

    fn partition(nums: &mut Vec<i32>, lo: usize, hi: usize) -> (usize, usize) {
        let pivot = nums[(lo + hi) / 2];

        let mut i = lo;
        let mut l = lo;
        let mut r = hi;

        while i <= r {
            if nums[i] > pivot {
                nums.swap(i, l);
                i += 1;
                l += 1;
            } else if nums[i] < pivot {
                nums.swap(i, r);
                r -= 1;
            } else {
                i += 1;
            }
        }
        (l, r)
    }
}
