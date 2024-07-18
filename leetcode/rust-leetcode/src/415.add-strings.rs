impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num1 = num1.into_bytes().into_iter().map(|b| (b-b'0') as u8).collect::<Vec<_>>();
        let mut num2 = num2.into_bytes().into_iter().map(|b| (b-b'0') as u8).collect::<Vec<_>>();
        num1.reverse();
        num2.reverse();
        let l1 = num1.len();
        let l2 = num2.len();
        let l = l1.max(l2);
        let mut sum = vec![0; l];
        let mut carry = 0;
        for i in 0..l {
            if i < l1 && i < l2 {
                sum[i] = num1[i] + num2[i] + carry;
            } else if i < l1 {
                sum[i] = num1[i] + carry;
            } else {
                sum[i] = num2[i] + carry;
            }
            if sum[i] > 9 {
                carry = 1;
                sum[i] -= 10;
            } else {
                carry = 0;
            }
        }
        if carry > 0 {
            sum.push(carry);
        }
        sum.reverse();
        sum.into_iter().map(|b| (b'0' + b) as char).collect()
    }
}

/* */

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {

        let max = if num1.len() < num2.len() {&num2} else {&num1};

        let mut res: Vec<u32> = num1.chars().rev().zip(num2.chars().rev()).map(|(x1, x2)| x1.to_digit(10).unwrap() + x2.to_digit(10).unwrap()).collect();
        println!("res {:?}", res);

    	let head = &max[0..(max.len()-res.len())];
        let head: Vec<u32> = head.chars().rev().map(|x| x.to_digit(10).unwrap()).collect();

        res.extend(head);
        Self::pluser(&mut res);
        println!("res {:?}", res);

        let fin_res = res.iter().rev().map(|x| x.to_string()).collect::<String>();
        return fin_res
        // return String::from("")


    }

    fn pluser(num: &mut Vec<u32>) {
        let len = num.len();
        if num[len-1] > 10 {
            num[len-1] -= 10;
            num.extend(vec![1])
        }

        for i in 0..num.len()-1 {
            if num[i] >= 10 {
                num[i] -= 10;
                num[i+1] += 1;
            }
        }
    }

}
