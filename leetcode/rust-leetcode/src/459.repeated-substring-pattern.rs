impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let sb = s.into_bytes();
        // Linux
        let l = sb.len();
        for i in 1..=l/2 {
            if l % i == 0 {
                let mut flag = true;
                for j in i..l {
                    if sb[j] != sb[j-i] {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return flag;
                }
            }
        }
        false
    }
}

/* */

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let prefixLen = Self::calc_prefix_len(&s);
        let repLen = s.len() - (*prefixLen.last().unwrap() as usize);
        // println!("{repLen}");
        repLen < s.len() && s.len() % repLen == 0
    }


    // calculating KMP prefix arr
    fn calc_prefix_len(pattern: &str) -> Vec<i32> {
        let patternLen = pattern.len();
        let mut arr = vec![0; patternLen+1];
        arr[0] = -1;
        
        let mut i = 1;
        let mut prefixLen = 0;

        let pb = pattern.to_string().into_bytes();

        while i < patternLen {
            if pb[prefixLen] == pb[i] {
                prefixLen += 1;
                i += 1;
                arr[i] = prefixLen as i32;
            } else if prefixLen > 0 {
                prefixLen = arr[prefixLen as usize] as usize;
            } else {
                i += 1;
                arr[i] = 0;
            }
        }
        arr
    }
}

