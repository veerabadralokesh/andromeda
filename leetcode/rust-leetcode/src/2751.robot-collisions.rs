impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let mut bots = Vec::with_capacity(healths.len());
        for (i, (p, (&h, d))) in positions.into_iter().zip(healths.iter().zip(directions.into_bytes())).enumerate() {
            bots.push(
                (if d == b'R' {1} else {-1}, p, h, i)
            );
        }
        bots.sort_by_key(|&b| b.1);
        // println!("{:?}", bots);
        let mut stack = Vec::with_capacity(healths.len());
        stack.push(bots[0]);
        let mut bot_alive = false;
        for (mut b) in bots.into_iter().skip(1) {
            bot_alive = true;
            while !stack.is_empty() && stack.last().unwrap().0 == 1 && b .0 == -1 {
                if let Some(mut rb) = stack.pop() {
                    if rb.2 == b.2 {
                        bot_alive = false;
                        break;
                    } else if rb.2 > b.2 {
                        rb.2 -= 1;
                        stack.push(rb);
                        bot_alive = false;
                        break;
                    } else {
                        b.2 -= 1;
                    }
                }
            }
            if bot_alive {
                stack.push(b);
            }
        }
        stack.sort_by_key(|&b| b.3);
        stack.into_iter().map(|b| b.2).collect()
    }
}

