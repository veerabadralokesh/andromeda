impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let binding = num.to_string();
        let mut numb = binding.bytes();
        let mut ans:i32 = 0;
        let mut flag:bool = true;
        for b in numb {
            if b == b'6' {
                if flag {
                    ans = ans * 10 + 9;
                    flag = false;
                } else {
                    ans = ans * 10 + 6;
                }
            } else {
                ans = ans * 10 + 9;
            }
        }
        ans
    }
}
