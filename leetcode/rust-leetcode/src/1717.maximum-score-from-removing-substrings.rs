impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut sb = s.into_bytes();
        
        let sub1 = [b'a', b'b'];
        let sub2 = [b'b', b'a'];

        let gain = |s1: [u8; 2], s2: [u8; 2], p1: i32, p2: i32| -> i32 {
            let mut stack1 = Vec::new();
            let mut p = 0;
            for &c in sb.iter() {
                if c == s1[1] && !stack1.is_empty() && *stack1.last().unwrap() == s1[0] {
                    p += p1;
                    stack1.pop();
                } else {
                    stack1.push(c);
                }
            }
            let mut stack2 = Vec::new();
            for &c in stack1.iter() {
                if c == s2[1] && !stack2.is_empty() && *stack2.last().unwrap() == s2[0] {
                    p += p2;
                    stack2.pop();
                } else {
                    stack2.push(c);
                }
            }
            p
        };
        
        if x > y {
            gain(sub1, sub2, x, y)
        } else {
            gain(sub2, sub1, y, x)
        }
    }
}


