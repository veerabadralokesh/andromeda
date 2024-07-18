impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut ss:Vec<String> = Vec::new();
        for i in 0..9 {
            ss.push("".to_string());
        }
        for word in s.split_whitespace() {
            let chars = word.chars();
            let i = (chars.clone().last().unwrap().to_digit(10).unwrap() - 1) as usize;
            let wi:String = chars.into_iter().take(word.len()-1).collect::<String>();
            ss[i] = wi;
        }
        ss.retain(|c| c.len() > 0);
        ss.join(" ").to_string()
    }
}

impl Solution2 {
    pub fn sort_sentence(s: String) -> String {
        let mut out = vec![];
        for word_num in s.split_whitespace() {
            let mut i = word_num.len();
            while i > 0 && word_num.as_bytes()[i - 1].is_ascii_digit() {
                i -= 1;
            }
            let word = &word_num[..i];
            let num = word_num[i..].parse::<usize>().unwrap() - 1;
            while num >= out.len() {
                out.push("");
            }
            out[num] = word;
        }
        out.join(" ")
    }
}

impl Solution3 {
    pub fn sort_sentence(s: String) -> String {
        let mut sentence = s.split_ascii_whitespace().collect::<Vec<_>>();
        sentence.sort_by(|&entry1, &entry2| {
            entry1
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap()
                .cmp(
                    &entry2
                        .chars()
                        .last()
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .unwrap(),
                )
        });
        sentence
            .iter()
            .map(|&word| &word[..=word.len() - 2])
            .collect::<Vec<_>>().join(" ")

    }
}
