
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum:i32 = nums.iter().sum();
        let n:i32 = nums.len() as i32;
        n * (n+1) / 2 - sum
    }
}


impl Solution2 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;

        (len * (len + 1) / 2) - nums.iter().sum::<i32>()
    }
}

impl Solution3 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sequence = nums.iter().fold(vec![0; nums.len() + 1], |mut acc, it| {
            acc[*it as usize] += 1;

            acc
        });

        let pos = sequence.iter().position(|n| *n == 0).unwrap_or_default();

        pos as i32
    }
}

impl Solution4 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ans:i32 = 0;
        for i in 0..nums.len() {
            ans ^= ((i+1) as i32);
            ans ^= nums[i];
        }
        ans
    }
}