impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 101];
        for &n in nums.iter() {counts[n as usize] += 1;}
        let (mut pairs, mut left) = (0,0);
        for c in counts {
            if c > 0 {
                pairs += c / 2;
                left += c % 2;
            }
        }
        vec![pairs,left]
    }
}

