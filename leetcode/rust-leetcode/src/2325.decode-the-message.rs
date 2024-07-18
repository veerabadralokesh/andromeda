use std::collections::HashMap;
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = HashMap::with_capacity(26);
        for c in key.chars() {
            if c == ' ' {continue;}
            if !map.contains_key(&c) {
                map.insert(c, (b'a' + (map.len() as u8)) as char);
                if map.len() == 26 {
                    break;
                }
            }
        }
        message.chars().map(|c| if c==' ' {c} else {*map.get(&c).unwrap()}).collect::<String>()
    }
}