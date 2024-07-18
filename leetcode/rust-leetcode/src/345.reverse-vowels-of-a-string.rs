impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut indices:Vec<usize> = Vec::new();
        for (i, b) in chars.clone().iter().enumerate() {
            if (
                *b == 'a' || *b == 'e' || *b=='i' || *b=='o' || *b=='u'
            || *b == 'A' || *b == 'E' || *b=='I' || *b=='O' || *b=='U'
            ) {
                indices.push(i);
            }
        }
        if indices.len() == 0 {
            return s;
        }
        let (mut i, mut j) = (0 as usize, (indices.len()-1) as usize);
        while i < j {
            let (x, y) = (chars[indices[i]], chars[indices[j]]);
            chars[indices[i]] = y;
            chars[indices[j]] = x;
            i += 1;
            j -= 1;
        }
        chars.iter().collect::<String>()
    }
}

impl Solution2 {
    pub fn reverse_vowels(s: String) -> String {
        let bytes = s.into_bytes();
        let vowels = bytes.iter().copied().filter(|&b| Self::is_vowel(b)).collect::<Vec<_>>();
        let mut vowel_it = vowels.into_iter().rev();
        let mut res = Vec::with_capacity(bytes.len());
        for byte in bytes {
            if Self::is_vowel(byte) {
                if let Some(v) = vowel_it.next() {
                    res.push(v);
                }
            } else {
                res.push(byte);
            }
        }
        unsafe {
            String::from_utf8_unchecked(res)
        }
    }
    fn is_vowel(byte: u8) -> bool {
        match byte {
            b'a' | b'e' | b'i' | b'o' | b'u' |
            b'A' | b'E' | b'I' | b'O' | b'U' => true,
            _ => false
        }
    }
    fn next_byte_back(bytes: &[u8]) -> Option<(usize, u8)> {
        bytes.into_iter().copied().enumerate().rev().filter(|&(i, byte)| Self::is_vowel(byte)).next()
    }
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

impl Solution3 {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels: Vec<char> = Vec::new();
        for c in s.chars() {
            if is_vowel(c) {
                vowels.push(c);
            }
        }
        let mut sBytes = s.into_bytes();
        for i in 0..sBytes.len() {
            if is_vowel(sBytes[i] as char) {
                sBytes[i] = vowels.pop().unwrap() as u8;
            }
        }
        return String::from_utf8(sBytes).unwrap();
    }
}
