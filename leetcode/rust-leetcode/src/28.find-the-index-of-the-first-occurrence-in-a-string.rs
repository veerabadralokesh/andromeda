impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(needle.as_str()) {
            Some(i) => i as i32,
            _ => -1
        }
    }
}
    
/*  */

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        
        let prefixLen = Self::calc_prefix_len(&needle);

        // println!("{:?}", prefixLen);
        let mut h = 0;
        let mut n = 0;

        let hlen = haystack.len();
        let nlen = needle.len();

        let mut matches = vec![];

        while h < hlen {
            if haystack[h..h+1] == needle[n..n+1] {
                h += 1;
                n += 1;

                if n == nlen {
                    return (h - n) as _;
                    matches.push((h - n) as i32);
                    n = 0;
                }
            } else if n == 0 {
                h += 1;
            } else {
                n = prefixLen[n] as usize;
            }
        }
        // println!("{:?}", matches);

        if matches.is_empty() {
            -1
        } else {
            matches[0]
        }
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

