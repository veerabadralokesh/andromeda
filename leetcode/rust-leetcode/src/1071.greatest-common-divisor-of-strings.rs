

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let s1 = str1.chars().collect::<Vec<char>>();
        let s2 = str2.chars().collect::<Vec<char>>();

        fn gcd(a:usize, b:usize) -> usize{
            let mut ans:usize = 1;
            let limit = a.min(b);
            for i in 2..(limit+1) {
                if a%i == 0 && b%i == 0 {
                    ans = i;
                }
            }
            return ans;
        }

        let g = gcd(s1.len(), s2.len());
        let mut common:Vec<char> = Vec::with_capacity(g);
        for i in 0..g {common.push(s1[i]);}
        let mut valid:bool = true;
        for i in 0..s1.len() {
            if s1[i] != common[i%g] {
                return "".to_string();
            }
        }
        for i in 0..s2.len() {
            if s2[i] != common[i%g] {
                return "".to_string();
            }
        }
        let mut ans: String = String::with_capacity(g);
        for i in 0..g {ans.push(s1[i]);}
        ans
    }
}

impl Solution2 {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut s1 = str1.clone();
        s1 += &str2.clone();

        let mut s2 = str2.clone();
        s2 += &str1.clone();

        if s1 != s2 {
            return String::from("");
        }

        str1.get(0..Self::gcd(str1.len(), str2.len())).unwrap().to_string()
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }
}

impl Solution3 {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // assume that str2 is shorter
        if str1 == str2 {
            return str1;
        }
        let (a, b) = if str1.len() > str2.len() {(str1, str2)} else {(str2,str1)};
        if b == "" {
            return "".to_string()
        } else if a.starts_with(&b) {
            return Solution::gcd_of_strings(a.strip_prefix(&b).unwrap().to_string(), b)
        } else {
            return "".to_string()
        }
    }
}