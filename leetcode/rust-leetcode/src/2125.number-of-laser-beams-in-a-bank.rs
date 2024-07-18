impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut counts = bank.iter().map(
            |w| w.clone().into_bytes().into_iter().filter(|b| *b == b'1').count() as i32
        ).collect::<Vec<i32>>();
        let mut prev_count = 0;
        let mut ans = 0i32;
        for c in counts {
            if c > 0 {
                ans += prev_count * c;
                prev_count = c;
            }
        }
        ans
    }
}

/*
*/

// LEARN

use std::num::NonZeroI32;
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .filter_map(|row_tiles| {
                NonZeroI32::new(
                    row_tiles
                        .as_bytes()
                        .into_iter()
                        .fold(0, |mut acc, value| acc + (*value == b'1') as i32),
                )
            })
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, |acc, entry| acc + entry[0].get() * entry[1].get())
    }
}
