impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut cv = vec![0; 26];
        for b in chars.bytes() {cv[(b-b'a') as usize] += 1;}
        let mut ans = 0i32;
        let mut flag = true;
        for word in words {
            let mut wv = cv.to_vec();
            for b in word.bytes() {
                let i = (b - b'a') as usize;
                if wv[i] == 0 {
                    flag = false;
                    break;
                }
                wv[i] -= 1;
            }
            if flag {
                ans += (word.len() as i32);
            }
            flag = true;
        }
        ans
    }
}
