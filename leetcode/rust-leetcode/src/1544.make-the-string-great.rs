impl Solution {
    pub fn make_good(s: String) -> String {
        let mut goods:Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            if goods.len() > 0 {
                let pcb = *goods.last().unwrap() as u8;
                let cb = c as u8;
                if pcb.saturating_sub(cb) == 32 || cb.saturating_sub(pcb) == 32 {
                    goods.pop();
                } else {
                    goods.push(c);
                }
            } else {
                goods.push(c);
            }
        }
        goods.iter().collect()
    }
}

/* */

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut buf: Vec<char> = Vec::new();

        for c in s.chars() {
            let last = buf.last();

            if last.is_some() {
                let l = *last.unwrap();         

                if l != c && l.to_ascii_lowercase() == c.to_ascii_lowercase() {
                    buf.pop();
                    continue;
                }
            }

            buf.push(c);
        }

        String::from_iter(buf)
    }
}
