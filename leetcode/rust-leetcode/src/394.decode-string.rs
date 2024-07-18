impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack:Vec<char> = Vec::new();
        let mut mult:u32 = 1;
        let mut iter:u32 = 0;
        for c in s.chars() {
            if c == ']' {
                let mut cv:Vec<char> = Vec::new();
                while stack[stack.len()-1] != '[' {
                    cv.push(stack.pop().unwrap());
                }
                stack.pop();
                cv.reverse();
                mult = 1; iter = 0;
                while stack[stack.len()-1].is_numeric() {
                    iter += mult * stack.pop().unwrap().to_digit(10).unwrap();
                    mult *= 10;
                    if stack.len() == 0 {
                        break;
                    }
                }
                for i in 0..iter {
                    stack.extend(cv.to_vec());
                }
            } else {
                stack.push(c);
            }
        }
        // println!("{:?}", stack);
        stack.iter().collect::<String>()
    }
}
