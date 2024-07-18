impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let mut ans = vec![0; s.len()];
        let sb = s.into_bytes();
        for i in 0..ans.len() {
            let (mut x, mut y) = (start_pos[0], start_pos[1]);
            let mut dist = 0;
            for j in i..ans.len() {
                match sb[j] {
                    b'U' => {
                        if x == 0 {break;}
                        x -= 1; dist += 1;
                    },
                    b'D' => {
                        if x == n-1 {break;}
                        x += 1; dist += 1;
                    },
                    b'L' => {
                        if y == 0 {break;}
                        y -= 1; dist += 1;
                    },
                    b'R' => {
                        if y == n-1 {break;}
                        y += 1; dist += 1;
                    },
                    _ => unreachable!(),
                }
            }
            ans[i] = dist;
        }
        ans
    }
}

