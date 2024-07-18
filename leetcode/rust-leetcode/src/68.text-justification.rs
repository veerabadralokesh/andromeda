impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut lengths = words.to_vec().into_iter().map(|x| x.len() as i32).collect::<Vec<i32>>();
        // println!("{:?}", lengths);
        let mut ans:Vec<String> = Vec::new();
        let mut i:usize = 0;
        let mut line_width:i32 = 0;
        let mut line_words:Vec<String> = Vec::new();
        let mut line = String::new();
        let mut add_spaces = 0;
        while i < lengths.len() {
            line_width = 0;
            while line_width + lengths[i] + (line_words.len() as i32) <= max_width {
                line_width += lengths[i];
                line_words.push(words[i].clone());
                i += 1;
                if i == words.len() {
                    break;
                }
            }
            if i != words.len() {
                add_spaces = max_width - line_width;
                // println!("{:?}, {line_width}, {add_spaces}", line_words);
                while add_spaces > 0 {
                    for i in 0..((line_words.len()-1).max(1)) {
                        line_words[i].push(' ');
                        add_spaces -= 1;
                        if !(add_spaces > 0) {
                            break;
                        }
                    }
                }
                for word in &line_words {
                    line.push_str(word.as_str());
                    // line.push(' ');
                }
            } else {
                add_spaces = max_width - line_width - (line_words.len() as i32) + 1;
                // println!("{:?}, {line_width}, {add_spaces}", line_words);
                // for word in &line_words {
                //     line.push_str(word.as_str());
                //     line.push(' ');
                // }
                line.push_str((line_words.join(" ")).as_str());
                for _ in 0..add_spaces{
                    line.push(' ');
                }
            }
            ans.push(line.clone());
            line_words.clear();
            line.clear();
        }
        ans
    }
}
