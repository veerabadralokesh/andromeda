impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let is_capital = |w: String| -> bool {
            let (mut caps, mut smalls, l) = (0, 0, w.len());
            let wb = w.into_bytes();
            for &b in wb.iter() {
                if b > b'Z' {
                    smalls += 1;
                } else {
                    caps += 1;
                }
            }
            caps == l || smalls == l || (caps == 1 && smalls == l-1 && wb[0] <= b'Z')
        };
        word.split_whitespace().all(|w| is_capital(w.to_string()))
    }
}

