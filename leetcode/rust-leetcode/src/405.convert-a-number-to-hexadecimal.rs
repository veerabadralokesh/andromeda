impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        let hex = "0123456789abcdef".chars().collect::<Vec<char>>();
        let mut numhex = Vec::new();
        let mut num = num as i64;
        if num < 0 {
            num += 2_i64.pow(32);
        }
        while num > 0 {
            numhex.push(hex[(num & 0xF) as usize]);
            num >>= 4;
        }
        numhex.reverse();
        numhex.into_iter().collect()
    }
}
