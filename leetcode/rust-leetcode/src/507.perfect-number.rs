impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let p = [2, 3, 5, 7, 13];//, 17, 19];
        let pn = p.iter().map(|&x| 2_i32.pow(x-1) * (2_i32.pow(x)-1)).collect::<Vec<_>>();
        // println!("{:?}", pn);
        pn.contains(&num)
    }
}

/* */

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }

        let mut result = 1;
        for div in 2..=f64::from(num).sqrt() as i32  {
            if num % div == 0 {
                result += (div + num / div);
            }
        }
        result == num
    }
}

