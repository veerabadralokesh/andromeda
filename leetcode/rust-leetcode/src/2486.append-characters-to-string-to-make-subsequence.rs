impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let sb = s.into_bytes();
        let tb = t.into_bytes();
        let mut sp = 0;
        let mut tp = 0;
        while tp < tb.len() && sp < sb.len() {
            if sb[sp] == tb[tp] {
                tp += 1;
            }
            sp += 1;
        }
        (tb.len()-tp) as i32
    }
}


