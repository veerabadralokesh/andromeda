impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut filtered_string: String = s
                        .to_lowercase()
                        .chars()
                        .filter(
            |letter| letter.is_alphanumeric()
                        ).collect();

        return filtered_string == filtered_string.chars().rev().collect::<String>()
    }
}

/*
*/

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() <= 1 {
            return true;
        }
        let slc = s.to_lowercase();
        let sb = slc.bytes().filter(|b| (
            ((b - b'a') >= 0 && (b - b'a') < 26) || ((b-b'0' >= 0 && b-b'0' <=9))
            )).collect::<Vec<u8>>();
        let l = sb.len();
        if l <= 0 {
            return true;
        }
        let mut sstring = String::new();
        for i in 0..l {
            if sb[i] != sb[l-i-1] {
                return false;
            }
        }
        true
    }
}
