impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        let mut tokens = tokens.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let is_op = |s: &str| s == "+" || s == "-" || s == "*" || s == "/";
        for token in tokens {
            if is_op(token) {
                let num2 = stack.pop().unwrap();
                let num1 = stack.pop().unwrap();
                if token == "+" {
                    stack.push(num1+num2);
                } else if token == "-" {
                    stack.push(num1-num2);
                } else if token == "*" {
                    stack.push(num1*num2);
                } else {
                    stack.push(num1/num2);
                }
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack[0]
    }
}

/* */

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());
        for token in tokens.into_iter() {
            match token.as_str() {
                "+" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b + a);
                },
                "-" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                },
                "*" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);
                },
                "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                },
                _ => stack.push(token.parse().unwrap()),
            }
        }
        stack.pop().unwrap()
    }
}
