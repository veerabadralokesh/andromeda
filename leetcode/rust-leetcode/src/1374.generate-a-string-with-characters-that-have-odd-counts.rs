impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut sc = vec!['a'; n as usize];
        if n % 2 == 0 {
            sc[0] = 'b';
        }
        sc.iter().collect()
    }
}

