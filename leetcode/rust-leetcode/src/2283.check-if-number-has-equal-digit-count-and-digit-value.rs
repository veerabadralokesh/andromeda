impl Solution {
    pub fn digit_count(num: String) -> bool {
        let nums = num.into_bytes().into_iter().map(|b| (b - b'0') as usize).collect::<Vec<_>>();
        let mut counts = [0; 10];
        for &n in nums.iter() {
            counts[n] += 1;
        }
        for i in 0..nums.len() {
            if counts[i] != nums[i] {
                return false;
            }
        }
        true
    }
}

