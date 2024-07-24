impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut i = 0;
        let mut num: i32 = 0;
        let mut sign: i32 = 1;

        let mut prev_op = None;

        for c in s.chars() {
            if c.is_digit(10) {
                num = 10*num + c.to_digit(10).unwrap() as i32;
            } else if c == '+' || c == '-' || c == '*' || c == '/' {
                if prev_op == Some('*') {
                    stack[i] *= (num * sign);
                } else if prev_op == Some('/') {
                    if (num * sign) == 0 {
                        break;
                    }
                    stack[i] /= (num * sign);
                } else {
                    stack.push(num*sign);
                }
                i = stack.len()-1;
                num = 0;
                sign = 1;
                if c == '*' || c == '/' {
                    prev_op = Some(c);
                } else if c == '+' || c == '-' {
                    sign = if c == '+' {1} else {-1};
                    prev_op = None;
                }
            }
        }
        if prev_op == Some('*') {
            stack[i] *= (num * sign);
        } else if prev_op == Some('/') {
            stack[i] /= (num * sign);
        } else {
            stack.push(num*sign);
        }

        stack.iter().sum::<i32>()
    }
}

