impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let chars = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z'],
        ];
        let digits = digits.into_bytes().iter().map(|b| (b-b'2') as usize).collect::<Vec<_>>();
        let n = digits.len();
        let mut spans = vec![1;n];
        let counts = digits.iter().map(|&d| chars[d].len()).collect::<Vec<_>>();
        for i in (0..digits.len()-1).rev() {
            spans[i] = spans[i+1]*counts[i+1];
        }
        let mut total = spans[0] * counts[0];
        let mut combinations = vec![String::new(); total];
        for i in 0..total {
            for j in 0..digits.len() {
                combinations[i].push(chars[digits[j]][(i/spans[j])%counts[j]]);
            }
        }
        combinations
    }
}

/* */

const MAPPING: [std::ops::RangeInclusive<u8>; 8] = [
    (b'a'..=b'c'),
    (b'd'..=b'f'),
    (b'g'..=b'i'),
    (b'j'..=b'l'),
    (b'm'..=b'o'),
    (b'p'..=b's'),
    (b't'..=b'v'),
    (b'w'..=b'z'),
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        digits.as_bytes().iter().fold(
            if digits.is_empty() {
                Vec::new()
            } else {
                vec![String::new()]
            },
            |acc, &x| {
                acc.iter()
                    .flat_map(|s| {
                        std::iter::repeat(s)
                            .zip(MAPPING[(x - b'2') as usize].clone())
                            .map(|(s, b)| s.chars().chain(std::iter::once(b as char)).collect())
                            .collect::<Vec<_>>()
                    })
                    .collect()
            },
        )
    }
}
