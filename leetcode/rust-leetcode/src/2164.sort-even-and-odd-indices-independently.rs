impl Solution {
    pub fn sort_even_odd(mut nums: Vec<i32>) -> Vec<i32> {
        // let (even, odd): (Vec<_>, Vec<_>) = nums.iter().partition(|&x| x % 2 == 0);
        // nums[..odd.len()].copy_from_slice(&odd);
        // nums[odd.len()..].copy_from_slice(&even);
        let mut even = [0; 101];
        let mut odd = [0; 101];
        for (i, &n) in nums.iter().enumerate() {
            if i & 1 == 0 {
                even[n as usize] += 1;
            } else {
                odd[n as usize] += 1;
            }
        }
        let mut idx = 0;
        for i in 0..101 {
            while even[i] > 0 {
                nums[idx] = i as i32;
                idx += 2;
                even[i] -= 1;
            }
        }
        idx = 1;
        for i in (0..101).rev() {
            while odd[i] > 0 {
                nums[idx] = i as i32;
                idx += 2;
                odd[i] -= 1;
            }
        }
        nums
    }
}

