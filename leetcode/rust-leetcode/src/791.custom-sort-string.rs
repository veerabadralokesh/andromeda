impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut indices = vec![0usize; 26];
        for (i, b) in order.bytes().enumerate() {
            indices[(b - b'a') as usize] = i;
        }
        let mut sc = s.chars().collect::<Vec<char>>();
        sc.sort_by_key(|c| indices[((*c as u8) - b'a') as usize]);
        // println!("{:?}", sc);
        sc.iter().collect::<String>()
    }
}
