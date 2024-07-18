impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut even, mut odd) = (0, 1);
        let mut i = 0;
        while i < nums.len() {
            if i & 1 == 0 {
                if nums[i] & 1 == 0 {
                    i += 1;
                } else {
                    nums.swap(i, odd);
                    odd += 2;
                }
            } else {
                if nums[i] & 1 == 1 {
                    i += 1;
                } else {
                    nums.swap(i, even);
                    even += 2;
                }
            }
        }
        nums
    }
}

