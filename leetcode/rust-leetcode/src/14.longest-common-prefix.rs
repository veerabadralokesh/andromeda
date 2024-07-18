impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        let mut lcp = String::new();
        strs.sort_by_key(|s| s.len());
        for i in 0..strs[0].len() {
            let c = strs[0].get(i..i+1).unwrap();
            let mut flag = true;
            for j in 1..strs.len() {
                if c != strs[j].get(i..i+1).unwrap() {
                    flag = false;
                    break;
                }
            }
            if flag {
                lcp.push_str(c);
            } else {
                break;
            }
        }
        lcp
    }
}

/*
*/

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let char_vecs: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
        let mut prefix = String::new();
        
        // Find the length of the shortest string
        let min_length = char_vecs.iter().map(|vec| vec.len()).min().unwrap_or(0);

        'outer: for i in 0..min_length {
            let c = char_vecs[0][i];  // First string's current character for comparison
            for s in &char_vecs[1..] {  // Start from second string
                if i >= s.len() || s[i] != c {
                    // Break the outer loop if character does not match or index is out of bounds
                    break 'outer;
                }
            }
            prefix.push(c);  // Add character to prefix if it matches all strings so far
        }

        prefix
    }
}
