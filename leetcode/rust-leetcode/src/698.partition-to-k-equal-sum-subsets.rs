impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {return true;}
        let total = nums.iter().sum::<i32>();
        if total % k != 0 {
            return false;
        }
        let subtotal = total/k;
        if nums.iter().any(|&n| n > subtotal) {
            return false;
        }
        let k = k as usize;
        nums.sort();
        nums.reverse();
        fn backtrack(nums: &Vec<i32>, sum: i32, i: usize, k: usize, bitmask: u32, subtotal: i32) -> bool {
            if k == 0 {
                return true;
            }
            if sum > subtotal {return false;}
            if sum == subtotal {
                return backtrack(nums, 0, 0, k-1, bitmask, subtotal);
            }
            for j in i..nums.len() {
                if (1 & (bitmask >> j)) == 1 {
                    continue;
                }
                if backtrack(nums, sum + nums[j], j + 1, k, (bitmask | (1 << j)), subtotal) {
                    return true;
                }
            }
            false
        }
        backtrack(&nums, 0, 0, k, 0, subtotal)
    }
}

