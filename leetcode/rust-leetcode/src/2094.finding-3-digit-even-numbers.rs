impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut counts = [0; 10];
        for d in digits {
            counts[d as usize] += 1;
        }
        let mut ans = Vec::with_capacity(900);
        for i in 1..10 {
            if counts[i] > 0 {
                counts[i] -= 1;
                let x = i * 100;
                for j in 0..10 {
                    if counts[j] > 0 {
                        counts[j] -= 1;
                        let y = x + j * 10;
                        for k in (0..10).step_by(2) {
                            if counts[k] > 0 {
                                ans.push((y + k) as i32);
                            }
                        }
                        counts[j] += 1;
                    }
                }
                counts[i] += 1;
            }
        }
        ans
    }
}

