impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        let mut ans = Vec::new();
        let mut neg = false;
        if num < 0 {
            num = -num;
            neg = true;
        }
        loop {
            ans.push(((num % 7) as u8 + b'0') as char);
            num /= 7;
            if num == 0 {
                break;
            }
        }
        if neg {ans.push('-');}
        ans.reverse();
        ans.iter().collect()
    }
}

