impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 2 {
            return nums;
        }
        let mut xor = nums[0];
        for i in 1..nums.len() {
            xor ^= nums[i];
        }
        let bit_mask = xor & -xor;
        // println!(" {xor}: {:#034b}\n{}: {:#034b}\n {bit_mask}: {:#034b} ", xor, -xor, -xor, bit_mask);
        let mut ans = vec![];
        let (mut a1, mut a2) = (0, 0);
        for &n in nums.iter() {
            // println!("{n}: {}", (n & bit_mask));
            if n & bit_mask > 0 {
                a1 ^= n;
            } else {
                a2 ^= n;
            }
        }
        ans.push(a1);
        ans.push(a2);
        ans
    }
}

/* */

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 2 {
            return nums;
        }
        let mut xor = nums[0];
        for i in 1..nums.len() {
            xor ^= nums[i];
        }
        // let bit_mask = xor & -xor;
        // println!(" {xor}: {:#034b}\n{}: {:#034b}\n {bit_mask}: {:#034b} ", xor, -xor, -xor, bit_mask);
        let mut bit_mask = 1;
        while xor & bit_mask == 0 {
            bit_mask <<= 1;
        }
        let mut ans = vec![];
        let (mut a1, mut a2) = (0, 0);
        for &n in nums.iter() {
            // println!("{n}: {}", (n & bit_mask));
            if n & bit_mask > 0 {
                a1 ^= n;
            } else {
                a2 ^= n;
            }
        }
        ans.push(a1);
        ans.push(a2);
        ans
    }
}


