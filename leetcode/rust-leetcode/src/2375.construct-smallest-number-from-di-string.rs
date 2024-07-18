impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut nums = "123456789".chars().collect::<Vec<char>>();
        let (mut i, mut count) = (0, 0);
        let pattern = pattern.chars().collect::<Vec<char>>();
        while i < n {
            count = 0;
            while i + count < n && pattern[i + count] == 'D' {
                count += 1;
            }
            if count > 0 {
                nums[i..=i+count].reverse();
            }
            i += count + 1;
        }
        String::from_iter(nums[..=n].iter())
    }
}

