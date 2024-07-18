impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count:i32 = 0;
        let mut flag:i32 = 1;
        for b in s.bytes() {
            if b == b'*' {
                count += flag;
            } else if b == b'|' {
                flag = 1 - flag;
            }
        }
        count
    }
}

impl Solution2 {
    pub fn count_asterisks(s: String) -> i32 {
        let mut can_count = true;
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for &ch in chars.iter() {
            match ch {
                '|' => can_count = !can_count,
                '*' => if can_count { count += 1 },
                _ => (),
            }
        }
        count
    }
}
