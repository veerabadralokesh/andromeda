impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let prefixLen = Self::calc_prefix_len(&s);
        // println!("{:?}", prefixLen);
        s[..*(prefixLen.last().unwrap()) as usize].to_string()
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

