impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counts = [0; 10];
        for b in num.into_bytes() {
            counts[(b - b'0') as usize] += 1;
        }
        let mut front = Vec::new();
        for i in (0..=9).rev() {
            if counts[i] > 1 {
                if i > 0 || front.len() > 0 {
                    let x = counts[i] / 2;
                    counts[i] -= 2 * x;
                    for _ in 0..x {front.push((i as u8 + b'0') as char);}
                }
            }
        }
        let mut ans = front.iter().collect::<String>();
        for i in (0..=9).rev() {
            if counts[i] > 0 {
                ans.push((i as u8 + b'0') as char);
                break;
            }
        }
        front.reverse();
        for c in front.into_iter() {ans.push(c);}
        ans
    }
}


