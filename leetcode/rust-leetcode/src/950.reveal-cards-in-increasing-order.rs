impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();
        let n = deck.len();
        if n <= 2 {
            return deck;
        }
        let mut reveal = Vec::with_capacity(n);
        for i in (0..n).rev() {
            reveal.push(deck[i]);
            if reveal.len() > 1 {
                reveal.rotate_right(2);
            }
        }
        if deck[0] != reveal[0] {
            reveal.rotate_left(1);
        }
        reveal
    }
}

/* */

// LEARN

use std::collections::VecDeque;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();
        let mut ans = VecDeque::new();
        while let Some(v) = deck.pop() {
            ans.push_front(v);
            ans.rotate_right(1);
        }
        ans.rotate_left(1);
        ans.into()
    }
}

