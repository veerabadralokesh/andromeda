impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut set = [0; 10];
        for i in 1..4 {
            let mut x = n * i;
            while x > 0 {
                set[(x % 10) as usize] += 1;
                x /= 10;
            }
        }
        if set[0] > 0 || (1..10).any(|i| set[i]!=1) {
            false
        } else {
            true
        }
    }
}

