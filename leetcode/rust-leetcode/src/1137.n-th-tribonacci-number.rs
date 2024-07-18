impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut track:Vec<i32> = vec![0, 1, 1];
        let mut ans:i32 = 0;
        for i in 3..(n+1) {
            ans = track[0] + track[1] + track[2];
            track[(i%3) as usize] = ans;
        }
        track[(n%3) as usize]
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut str = SolutionTribonacci::default();
        str.tribonacci(n)
    }
}
#[derive(Default)]
struct SolutionTribonacci {
    cache:HashMap<i32, i32>
}
impl SolutionTribonacci {
    pub fn tribonacci(&mut self, n: i32) -> i32 {
        match n {
            0 => { 0 }
            1 => { 1 }
            2 => { 1 }
            _ => {
                //println!("{n}");
                if self.cache.contains_key(&n) {
                    return *self.cache.get(&n).unwrap();
                } else {
                    let result = self.tribonacci(n - 3) + self.tribonacci(n - 2) + self.tribonacci(n - 1);
                    self.cache.insert(n, result);
                    result
                }
            }
        }
    }
}
