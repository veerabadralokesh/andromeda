impl Solution {
    pub fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
        let mut ans = 0;
        let n = matrix[0].len();
        let matrix = matrix.iter().map(|row| {
            let mut r = 0;
            for &rc in row.iter() {
                r = r * 2 + rc;
            }
            r
        }).collect::<Vec<_>>();
        fn backtrack(m: &Vec<i32>, start: usize, r: usize, mask: i32, ans: &mut i32, n: usize) {
            if r == 0 {
                let mut count = 0;
                for &r in m.iter() {
                    if r & mask == r {
                        count += 1;
                    }
                }
                *ans = (*ans).max(count);
                return;
            }
            for i in start..(n-r+1) {
                if (mask & (1 << i) == 0) {
                    backtrack(m, i + 1, r-1, (mask | (1 << i)), ans, n);
                }
            }
        }
        backtrack(&matrix, 0, num_select as usize, 0, &mut ans, n);
        ans
    }
}

