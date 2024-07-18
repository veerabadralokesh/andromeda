impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1i = num1.clone().bytes().map(|b| b - b'0').collect::<Vec<_>>();
        let num2i = num2.bytes().map(|b| b - b'0').collect::<Vec<_>>();
        let mut ans = vec![0; num1i.len()+num2i.len()];
        let mut resultij = 0;
        for i in (0..num1i.len()).rev() {
            for j in (0..num2i.len()).rev() {
                resultij = ans[i+j+1] + num1i[i] * num2i[j];
                ans[i+j] += resultij/10;
                ans[i+j+1] = resultij % 10;
            }
        }
        let mut start_index = ans.len()-1;
        for i in 0..ans.len() {
            if ans[i] != 0 {
                start_index = i;
                break;
            }
        }

        ans[start_index..].iter().map(|&b| (b+b'0') as char).collect::<String>()
    }
}

/* */

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut rv = [0u8; 400];

        for (i, a) in num2.as_bytes().iter().map(|v| *v - b'0').rev().enumerate() {
            for (j, b) in num1.as_bytes().iter().map(|v| *v - b'0').rev().enumerate() {
                let res = a * b + rv[i + j];
                rv[i + j] = res % 10;

                let mut carry = res / 10;
                let mut k = i + j + 1;
                while carry > 0 {
                    let t = carry + rv[k];
                    rv[k] = t % 10;
                    carry = t / 10;
                    k += 1;
                }
            }
        }

        let pos = match rv.iter().rev().position(|v| *v != 0) {
            Some(p) => rv.len() - p,
            None => 1,
        };


        String::from_utf8(rv[..pos].iter().rev().map(|ch| *ch + b'0').collect()).unwrap()
    }
}
