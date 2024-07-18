impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if num.len() == (k as usize) {
            return String::from("0");
        }
        let mut ans = String::new();
        let mut k = k;
        let numc = num.chars().collect::<Vec<_>>();
        let mut stack = Vec::new();
        for (i, c) in numc.iter().enumerate() {
            while stack.len() > 0 && *stack.last().unwrap() > c && k > 0 {
                stack.pop();
                k -= 1;
            }
            stack.push(c);
        }
        while k > 0 {
            stack.pop();
            k -= 1;
        }
        for &c in stack.into_iter() {
            if c != '0' || ans.len() != 0 {
                ans.push(c);
            }
        }
        if ans.len() == 0 {
            return String::from("0");
        }
        ans
    }
}

/* */

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut stack: Vec<char> = Vec::new();

        for c in num.chars() {
            while !stack.is_empty() && k > 0 && c < *stack.last().unwrap() {
                stack.pop();
                k -= 1;
            }

            stack.push(c);
        }

        // Remove remaining digits
        for _ in 0..k {
            stack.pop();
        }

        // Construct the smallest possible integer
        let mut result = stack.into_iter().collect::<String>();

        // Remove leading zeros
        result = result.trim_start_matches('0').to_string();
        if result.is_empty() {
            return "0".to_string();
        }

        result
    }
}


/* */

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        let mut stack: Vec<char> = vec![];
        for ch in num.chars() {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > ch {
                stack.pop();
                k -= 1;
            }
            stack.push(ch);
        }
        let mut res = stack
            .iter()
            .skip_while(|ch| **ch == '0')
            .collect::<String>();
        for _ in 0..k {
            res.pop();
        }
        if res.is_empty() {
            res.push('0');
        }
        res
    }
}
