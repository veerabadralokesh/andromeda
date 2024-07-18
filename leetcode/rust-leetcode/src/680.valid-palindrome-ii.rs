impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let sb = s.into_bytes();
        let l = sb.len();
        fn check_palindrome(sb: &Vec<u8>, start:usize, end:usize) -> bool {
            for i in start..=(start+end)/2 {
                if sb[i] != sb[end-i+start] {
                    return false;
                }
            }
            true
        }
        
        for i in 0..l/2 {
            if sb[i] != sb[l-i-1] {
                return check_palindrome(&sb, i+1, l-1-i) || check_palindrome(&sb, i, l-2-i);
            }
        }
        true
    }
}

