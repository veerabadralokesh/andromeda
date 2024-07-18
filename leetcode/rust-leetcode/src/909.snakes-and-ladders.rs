impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (board.len(), board[0].len());
        let mut q = vec![];
        let mut steps = 0;
        let mut flag = vec![0; n * n + 1];

        q.push(1);
        while !q.is_empty() {
            let mut temp = vec![];
            while let Some(u) = q.pop() {
                if u == n * n {
                    return steps;
                }
                let mut v = u + 1;
                while v <= u + 6 && v <= n * n {
                    let (i, mut j) = (m - 1 - (v - 1) / n, (v - 1) % n);
                    if (m - 1 - i) % 2 != 0 {
                        j = n - 1 - j;
                    }
                    let w = board[i][j];
                    if w != -1 {
                        if flag[w as usize] == 0 {
                            temp.push(w as usize);
                        }
                        flag[w as usize] = 1;
                    } else {
                        if flag[v] == 0 {
                            temp.push(v);
                        }
                        flag[v] = 1;
                    }
                    v += 1;
                }
            }
            q = temp;
            steps += 1;
        }

        -1
    }
}

