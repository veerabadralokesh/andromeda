impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let (mut count1, mut count2, mut c1, mut c2, mut ans) = ([0; 26], [0; 26], 0, 0, 0);
        let sbi = s.into_bytes().iter().map(|&b| (b-b'a') as usize).collect::<Vec<_>>();
        for &bi in sbi.iter() {
            count2[bi] += 1;
        }
        c2 = count2.iter().filter(|&c| *c > 0).count();
        for &bi in sbi.iter() {
            if count1[bi] == 0 {
                c1 += 1;
                count1[bi] = 1;
            }
            count2[bi] -= 1;
            if count2[bi] == 0 {
                c2 -= 1;
            }
            if c1 == c2 {
                ans += 1;
            } else if ans > 0 {
                break;
            }
        }
        ans
    }
}

