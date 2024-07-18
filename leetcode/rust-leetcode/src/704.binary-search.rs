impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match(nums.binary_search(&target)) {
            Ok(i) => i as i32,
            Err(e) => -1,
        }
    }
}

/* */


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l <= r {
        let mid = (l + r) / 2;
        let v = nums[mid as usize];

        if v == target {
            return mid;
        } else if v < target {
            l = mid + 1;
        } else if v > target {
            r = mid - 1;
        }
    }

    -1

    }
}
