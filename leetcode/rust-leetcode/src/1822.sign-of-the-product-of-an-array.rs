impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for &n in nums.iter() {
            if n == 0 {return 0;}
            if n < 0 {
                sign *= -1;
            }
        }
        sign
    }
}

/* */

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for &n in nums.iter() {
            if n == 0 {return 0;}
            if n < 0 {
                count += 1;
            }
        }
        if count & 1 == 0 {1} else {-1}
    }
}


