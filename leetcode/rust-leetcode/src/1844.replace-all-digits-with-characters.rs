impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut answer = vec![];
        let mut prev_byte:u8 = 0;
        let mut flag:bool = false;
        for byte in s.as_bytes() {
            if flag {
                flag = false;
                answer.push((*byte + prev_byte - 48) as char);   
            } else {
                flag = true;
                prev_byte = *byte;
                answer.push(*byte as char);
            }
        }
        answer.into_iter().collect::<String>()
    }
}

impl Solution2 {
    fn shift(a : char, b: char) -> char {
        (a as u8 + b.to_digit(10).unwrap() as u8) as char
    }
    pub fn replace_digits(s: String) -> String {
        let mut s_arr = s.chars().collect::<Vec<char>>();
        let mut ret = String::new();
        for i in 0..s_arr.len() {
            if i % 2 == 1{
                ret.push(Self::shift(s_arr[i-1], s_arr[i]));
            } else {
                ret.push(s_arr[i]);
            }
            //println!("{:?}", ret);
        }
        return ret;
    }
}

impl Solution3 {
    pub fn replace_digits(s: String) -> String {
        let mut list=s.chars().collect::<Vec<_>>();
        let mut ans=String::new();
        for i in (0..list.len()){
            if list[i].is_alphabetic(){
                ans.push(list[i]);
            }else{
                let diff=(list[i] as u8-b'0');
                let letter=(list[i-1] as u8+diff) as char;
                ans.push(letter);
            }
        }
        ans
    }
}