use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut string = String::with_capacity((a + b + c) as usize);
        let mut heap = BinaryHeap::with_capacity(3);

        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut total = a + b + c;
        let mut last = None;

        while let Some((mut count, ch)) = heap.pop() {
            string.push(ch);
            count -= 1;
            total -= 1;

            if count > (total - count) * 2 {
                count -= 1;
                total -= 1;
                string.push(ch);
            }

            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }

        string
    }
}

/* */

use std::cmp::min;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        Self::helper(a as usize, b as usize, c as usize, 'a', 'b', 'c')
    }

    fn helper(a: usize, b: usize, c: usize, A: char, B: char, C: char) -> String {
        if a < b {
            return Self::helper(b, a, c, B, A, C);
        }
        if b < c {
            return Self::helper(a, c, b, A, C, B);
        }
        if b == 0 {
            return A.to_string().repeat(min(a, 2));
        }
        let mut ans = String::with_capacity((a + b + c) as usize);
        let aCount = min(a, 2);
        let bCount = if (a - aCount >= b) {1} else {0};
        for _ in 0..aCount {
            ans.push(A);
        }
        for _ in 0..bCount {
            ans.push(B);
        }
        ans.push_str(Self::helper(a - aCount, b - bCount, c, A, B, C).as_str());
        ans
    }
}

