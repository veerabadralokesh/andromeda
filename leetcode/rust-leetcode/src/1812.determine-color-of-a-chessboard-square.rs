impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let c = coordinates.as_bytes();
        ((c[0]-b'a') + (c[1]-b'0')) % 2 == 0
    }
}

/* */

// LEARN

impl Solution2 {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.as_bytes().into_iter().map(|c| *c % 2).sum::<u8>() == 1
    }
}
