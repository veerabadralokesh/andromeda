impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec![String::from("()")];
        }
        let mut ans = Vec::new();
        fn generate(ans: &mut Vec<String>, lc: i32, rc: i32, current_string: String) {
            if lc == 0 && rc == 0 {
                ans.push(current_string);
                return
            }
            if lc > 0 {
                let mut new_string = current_string.clone();
                new_string.push('(');
                generate(ans, lc - 1, rc, new_string);
            }
            if lc < rc {
                let mut new_string = current_string.clone();
                new_string.push(')');
                generate(ans, lc, rc-1, new_string);
            }
        }
        generate(&mut ans, n, n, String::new());
        ans
    }
}

/* */

// LEARN

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize * 2;
        let mut rv = Vec::new();
        let mut str = [0; 16];

        let mut stack = vec![(0, 1)];

        while !stack.is_empty() {
            let (i, e) = stack.pop().unwrap();
            str[i] = e;

            let sum = str[..=i].iter().fold(0, |c, &x| c + x);

            if sum < 0 {
                continue;
            }

            if i == n - 1 {
                if sum == 0 {
                    let t = str[..n]
                        .iter()
                        .map(|&ch| if ch == 1 { '(' } else { ')' })
                        .collect();

                    rv.push(t);
                }

                continue;
            }

            stack.push((i + 1, -1));
            stack.push((i + 1, 1));
        }

        rv
    }
}
