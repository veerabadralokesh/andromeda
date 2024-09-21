impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut sb = s.chars().collect::<Vec<_>>();
        sb.reverse();
        let mut rs = sb.iter().collect::<String>();
        for i in 0..rs.len() {
            if rs[i..] == s[..sb.len()-i] {
                let mut ans = rs[..i].to_string();
                ans.push_str(s.as_str());
                return ans;
            }
        }

        rs.push_str(s.as_str());
        rs
    }
}

/* */

// learn

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        
        let prime = 31;
        let mut straight_hash = 0;
        let mut reverse_hash = 0;
        let mut cur_pow = 1;

        let mut palindrome_ends_at: isize = -1;

        for (i, c) in s.chars().enumerate() {

            straight_hash = straight_hash + (c as i32) * cur_pow;
            reverse_hash = (c as i32) + reverse_hash * prime;
    
            if (straight_hash == reverse_hash)
            {
                // Note that we're not doing a break here.
                // That's because we need to find the _last_
                // equality, not the first one.
                
                palindrome_ends_at = i as isize;
            }
    
            cur_pow *= prime;
        }

        // Now forming the answer
        let take_count = (s.len() as isize - palindrome_ends_at - 1) as usize;
        let reversed_suffix: String = s.chars().rev().take(take_count).collect();

        reversed_suffix + &s
    }
}

