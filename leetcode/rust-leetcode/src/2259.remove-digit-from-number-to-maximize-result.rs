impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let nc = number.chars().collect::<Vec<_>>();
        let indices = nc.iter().enumerate().filter(|&(i, &c)| c == digit).map(|(i, &c)| i).collect::<Vec<_>>();
        let mut ans = String::from(&number[..indices[0]]);
        ans.push_str(&number[indices[0]+1..]);
        let mut s:String;
        for &i in indices.iter().skip(1) {
            s = String::from(&number[..i]);
            s.push_str(&number[i+1..]);
            if s > ans {
                ans = s;
            }
        }
        ans
    }
}

