impl Solution {
    pub fn get_smallest_string(n: i32, mut k: i32) -> String {
        let mut nums = vec![1; n as usize];
        k -= n;
        for i in (0..n as usize).rev() {
            if k > 25 {
                nums[i] += 25;
                k -= 25;
            } else {
                nums[i] += k;
                break;
            }
        }
        nums.into_iter().map(|n| (n as u8 - 1 + b'a') as char).collect()
    }
}

