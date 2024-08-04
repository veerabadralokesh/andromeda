use std::collections::HashMap;
static moves: [(i32, i32); 8] = [(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        fn recurse(n: i32, k: i32, r: i32, c: i32, memo: &mut HashMap<(i32, i32, i32),f64>) -> f64 {
            if k == 0 {
                return 1.0;
            }
            match memo.get(&(r, c, k)) {
                Some(count) => *count,
                None => {
                    let mut count = 0.0;
                    let (mut x, mut y) = (0, 0);
                    for m in moves.iter() {
                        x = r + m.0;
                        y = c + m.1;
                        if x > -1 && x < n && y > -1 && y < n {
                            count += recurse(n, k-1, x, y, memo);
                        }
                    }
                    count /= 8.0;
                    memo.insert((r, c, k), count);
                    count
                }
            }
        }
        let mut memo = HashMap::new();
        recurse(n, k, row, column, &mut memo)
    }
}

