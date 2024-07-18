impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 1;
        for &n in nums.iter() {
            left = left.max(n);
            right += n;
        }

        let get_num_groups = |max_sum| -> i32 {
            let mut groups = 1;
            let mut group_sum = 0;
            for &n in nums.iter() {
                if group_sum + n <= max_sum {
                    group_sum += n;
                } else {
                    groups += 1;
                    group_sum = n;
                }
            }
            groups
        };

        while left < right {
            let mid = left + (right - left) / 2;
            if get_num_groups(mid) > k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

