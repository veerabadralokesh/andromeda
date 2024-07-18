use std::collections::HashSet;
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let dict:HashSet<String> = dictionary.into_iter().collect();
        let chars = sentence.chars().collect::<Vec<_>>();
        let mut word = String::new();
        let mut ans = String::new();
        let mut i = 0;
        let l = chars.len();
        while i < l {
            let c = chars[i];
            if c != ' ' {
                word.push(c);
            }
            if c == ' ' || dict.contains(&word) || i == l-1 {
                ans.push_str(&word);
                word.clear();
                if c != ' ' {
                    while i < l && chars[i] != ' ' {
                        i += 1;
                    }
                }
                if i < l {
                    ans.push(' ');
                }
            }
            i += 1;
        }
        ans
    }
}

/* */

#[derive(Default)]
struct Trie {
    word: Option<String>,
    children: [Option<Box<Trie>>; 26],
}

impl Solution {
pub fn replace_words(words: Vec<String>, sentence: String) -> String {
    let mut root = Trie::default();

    for word in words.into_iter() {
        let mut curr = &mut root;

        for c in word.chars() {
            let c = c as usize - 'a' as usize;

            curr = curr.children[c].get_or_insert_with(Default::default);
        }

        curr.word = Some(word);
    }

    let mut words = sentence.split_ascii_whitespace();

    words.map(|word| {
        let mut curr = &root;

        for c in word.chars() {
            let c = c as usize - 'a' as usize;

            if let Some(next) = curr.children[c].as_ref() {
                curr = next;

                if let Some(word) = next.word.as_ref() {
                    return word.as_ref();
                }
            }
            else {
                break;
            }
        }

        word
    }).collect::<Vec<&str>>().join(" ")
}
}

