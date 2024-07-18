
// https://en.wikipedia.org/wiki/Pentatope_number
impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        (n + 1) * (n + 2) * (n + 3) * (n + 4) / 24
    }
}

/*
*/

impl Solution {
    fn string_count_from_set(set_size: u32, string_size: u32) -> u64 {
        let mut v = Vec::<u64>::new();
        for i in 1..=set_size {
            v.push(i as u64);
        }
        for _ in 1..string_size {
            for i in 1..set_size {
                let idx = i as usize;
                v[idx] += v[idx - 1];
            }
        }
        v[set_size as usize - 1]
    }
    pub fn count_vowel_strings(n: i32) -> i32 {
        Self::string_count_from_set(5, n as u32) as i32
    }
}
