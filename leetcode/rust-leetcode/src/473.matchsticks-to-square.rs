impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let mut total = matchsticks.iter().sum::<i32>();
        if total % 4 != 0 {
            return false;
        }
        matchsticks.sort();
        matchsticks.reverse();
        fn backtrack(m: &Vec<i32>, k: i32, i: usize, subtotal: i32, bitmask: u32, sum: i32) -> bool {
            if k == 0 {
                return true;
            }
            if sum > subtotal {
                return false;
            }
            if sum == subtotal {
                return backtrack(m, k-1, 0, subtotal, bitmask, 0);
            }
            for j in i..m.len() {
                if (bitmask >> j) & 1 == 1 {
                    continue;
                }
                if backtrack(m, k, j+1, subtotal, (bitmask | 1 << j), sum + m[j]) {
                    return true;
                }
            }
            false
        }
        backtrack(&matchsticks, 4, 0, total / 4, 0, 0)
    }
}

