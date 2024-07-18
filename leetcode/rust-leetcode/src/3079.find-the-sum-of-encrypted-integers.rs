impl Solution {
    pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
        let encrypt = |mut n: i32| {
            let mut max = 1;
            let mut mult = 0;
            while n > 0 {
                max = max.max(n % 10);
                mult = mult * 10 + 1;
                n /= 10;
            }
            max * mult
        };
        nums.iter().map(|&n| encrypt(n)).sum()
    }
}

