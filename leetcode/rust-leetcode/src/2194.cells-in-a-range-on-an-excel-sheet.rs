impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut ans:Vec<String> = Vec::new();
        // let c1:char = s.chars()[0];
        // let c2:char = s.chars()[3];
        // let r1:char = s.chars()[3];
        let s = s.chars().map(|c| c as u8).collect::<Vec<u8>>();
        let (c1, r1, c2, r2) = (s[0], s[1]-48, s[3], s[4]-48);

        // let c = char::from_digit(num: u32, radix: u32)
        // const RADIX:u32 = 10;
        // const num:u32 = 9;
        // let c:char = char::from_digit(num, RADIX).unwrap();
        // println!("{c1}, {}, {c2}, {}, {}, {}, {}", r1, r2, c, c1 as char, c2 as char);
        
        for c in c1..c2+1 {
            for r in r1..r2+1 {
                let mut s1 = String::new();
                s1.push(c as char);
                s1.push_str(&(r.to_string()));
                ans.push(s1);
            }
        }
        // println!("{:?}", ans);

        ans
    }
}

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let s_arr = s.into_bytes();
        let (c0, c1) = (s_arr[0], s_arr[3]);
        let (r0, r1) = (s_arr[1], s_arr[4]);
        let mut ret = Vec::new();
        for c in c0..=c1 {
            for r in r0..=r1 {
                ret.push(format!("{}{}", c as char, r as char));
            }
        }
        return ret;
    }
}
