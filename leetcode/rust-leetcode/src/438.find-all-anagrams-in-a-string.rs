impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {return vec![];}
        let l = p.len();
        let mut pcount = [0; 26];
        for b in p.into_bytes() {
            pcount[(b - b'a') as usize] += 1;
        }
        let mut scount = [0; 26];
        let mut count = 0;
        for i in 0..26 {
            if pcount[i] == 0 {
                count += 1;
            }
        }
        let mut ans = Vec::new();
        let sb = s.into_bytes().iter().map(|&b| (b - b'a') as usize).collect::<Vec<_>>();
        for i in 0..l {
            scount[sb[i]] += 1;
        }
        for i in l..sb.len() {
            if scount == pcount {
                ans.push((i-l) as i32);
            }
            scount[sb[i]] += 1;
            scount[sb[i-l]] -= 1;
        }
        if scount == pcount {
            ans.push((sb.len()-l) as i32);
        }
        ans
    }
}

