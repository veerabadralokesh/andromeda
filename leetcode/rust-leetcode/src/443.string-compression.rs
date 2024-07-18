impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut count:i32 = 1;
        let mut s:String = String::new();
        for i in 0..chars.len() {
            if i < chars.len()-1 && chars[i] == chars[i + 1] {
                count += 1;
            } else {
                s.push(chars[i]);
                if count > 1 {
                    for c in count.to_string().chars() {
                        s.push(c);
                    }
                }
                count = 1;
            }
        }
        for (i, c) in s.chars().enumerate() {
            chars[i] = c;
        }
        s.len() as i32
    }
}