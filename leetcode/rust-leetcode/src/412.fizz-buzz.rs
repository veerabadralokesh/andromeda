impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut starr: Vec<String> = Vec::with_capacity(n as usize);
        for i in 1..n+1 {
            if i % 15 == 0 {
                starr.push("FizzBuzz".to_string());
            } else if i % 3 == 0 {
                starr.push("Fizz".to_string());
            } else if i % 5 == 0 {
                starr.push("Buzz".to_string());
            } else {
                starr.push(i32::to_string(&i));
            }
        }
        starr
    }
}

impl Solution2 {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => result.push("FizzBuzz".to_string()),
                (0, _) => result.push("Fizz".to_string()),
                (_, 0) => result.push("Buzz".to_string()),
                _ => result.push(i.to_string()),
            }
        }
        return result
    }
}