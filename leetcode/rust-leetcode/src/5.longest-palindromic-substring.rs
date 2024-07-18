use std::collections::VecDeque;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let l = s.len();
        let bytes = s.clone().into_bytes();
        let mut dp = VecDeque::new();
        let mut longest = (0 as usize, 0 as usize);
        let mut maxl = 1;
        for i in 0..l {dp.push_back((i, i))};
        for i in 1..l {
            if bytes[i] == bytes[i-1] {
                dp.push_back((i-1, i));
                longest = (i-1, i);
                maxl = 2;
            }
        }
        while dp.len() > 0 {
            let (start, end) = dp.pop_front().unwrap();
            if start > 0 && end < (l-1) {
                if bytes[start-1] == bytes[end+1] {
                    dp.push_back((start-1, end+1));
                    let ssl = end - start + 3;
                    if ssl > maxl {
                        maxl = ssl;
                        longest = (start-1, end+1);
                    }
                }
            }
        }
        s.chars().skip(longest.0).take((longest.1-longest.0+1)).collect::<String>()
    }
}

/*
*/

impl Solution {

    pub fn longest_palindrome(s: String) -> String {  
        let bytes = s.as_bytes();  
        let mut dp = vec![vec![false; s.len()]; s.len()];  
        let (mut max_len, mut start) = (0, 0);  
      
        for i in (0..s.len()).rev() {  
            for j in i..s.len() {  
                if bytes[i] == bytes[j] && (j - i < 3 || dp[i + 1][j - 1]) {  
                    dp[i][j] = true;  
                    if j - i + 1 > max_len {  
                        max_len = j - i + 1;  
                        start = i;  
                    }  
                }  
            }  
        }  
        s[start..start + max_len].to_string()  
    }  
    
}

/*
*/

// use std::collections::HashSet;

impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        let mut result = "";
        for i in 0..s.len() {
            for j in i..=(i+1) {
                let new_result = expand_palindrome(&s, i, j);
                if new_result.len() > result.len() {
                    result = new_result;
                }
            }
        }
        result.into()
    }
}

fn expand_palindrome(s: &str, mut i: usize, mut j: usize) -> &str {
    while i > 0 && j < s.len() && s.chars().nth(i-1) == s.chars().nth(j) {
        i -= 1;
        j += 1;
    }

    &s[i..j]
}
