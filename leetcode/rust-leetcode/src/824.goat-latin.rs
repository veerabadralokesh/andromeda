impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        sentence.split_whitespace().enumerate().map(|(i,w)| {
            let mut mw = if "AEIOUaeiou".contains(&w[0..1]) {
                String::from(w)
            } else {
                let mut ss = String::from(&w[1..]);
                ss.push_str(&w[0..1]);
                ss
            };
            mw.push_str("ma");
            for _ in 0..i+1 {
                mw.push('a');
            }
            mw
        }).collect::<Vec<_>>().join(" ")
    }
}

