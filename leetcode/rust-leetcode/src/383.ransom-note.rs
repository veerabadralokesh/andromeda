impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_counts = [0; 26];

        for ch in magazine.bytes() {
            char_counts[(ch - b'a') as usize] += 1;
        }

        for ch in ransom_note.bytes() {
            let index = (ch - b'a') as usize;
            if char_counts[index] == 0 {
                return false;
            }
            char_counts[index] -= 1;
        }
        true
    }
}

impl Solution2 {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dict = vec![0; 32];
        // build the dictionary
        for b in magazine.into_bytes() {
            dict[b as usize % 32] += 1;
        }
        for n in ransom_note.into_bytes() {
            if dict[n as usize % 32] <= 0 {
                return false;
            }else {
                dict[n as usize % 32] -= 1;
            }
        }
        true
    }
}