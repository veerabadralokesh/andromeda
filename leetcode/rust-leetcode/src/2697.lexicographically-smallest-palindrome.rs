impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut sc = s.chars().collect::<Vec<_>>();
        let (mut left, mut right) = (0, sc.len() - 1);
        while left < right {
            if sc[left] < sc[right] {
                sc[right] = sc[left];
            } else if sc[left] > sc[right] {
                sc[left] = sc[right];
            }
            left += 1;
            right -= 1;
        }
        sc.iter().collect()
    }
}

