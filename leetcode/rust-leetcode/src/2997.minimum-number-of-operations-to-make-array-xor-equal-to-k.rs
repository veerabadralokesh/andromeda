impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor = 0;
        for n in nums {
            xor ^= n;
        }
        (xor ^ k).count_ones() as i32
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut xor = k;
        for n in nums {
            xor ^= n;
        }
        xor.count_ones() as i32
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, mut k: i32) -> i32 {
        for n in nums {
            k ^= n;
        }
        k.count_ones() as i32
    }
}


// LEARN
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().fold(k, |acc, &x| acc ^ x).count_ones() as i32
    }
}
