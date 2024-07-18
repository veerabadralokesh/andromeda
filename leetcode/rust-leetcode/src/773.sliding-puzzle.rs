use std::collections::HashSet;
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let solved = vec![1,2,3,4,5,0];
        let board = board.into_iter().flatten().collect::<Vec<_>>();
        if board == solved {
            return 0;
        }
        let swaps = vec![vec![1, 3], vec![0, 2, 4], vec![1, 5], vec![0, 4], vec![1, 3, 5], vec![2, 4]];
        let zero_idx = board.iter().position(|&r| r == 0).unwrap();
        let mut q = Vec::new();
        let mut visited = HashSet::with_capacity(720);
        visited.insert(board.to_vec());
        q.push((board.to_vec(), zero_idx));
        let mut moves = 0;
        while !q.is_empty() {
            let mut temp = Vec::new();
            for _ in 0..q.len() {
                if let Some((mut b, idx)) = q.pop() {
                    if b == solved {
                        return moves;
                    }
                    visited.insert(b.to_vec());
                    for &swap in swaps[idx].iter() {
                        let mut b_new = b.to_vec();
                        b_new.swap(idx, swap);
                        if visited.contains(&b_new) {
                            continue;
                        }
                        temp.push((b_new, swap));
                    }
                }
            }
            q = temp;
            moves += 1;
        }
        -1
    }
}

