use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut hs = HashSet::new();
        let mut n = n;
        hs.insert(n);
        let mut temp = 0;
        let mut x = 0;
        while n > 3 {
            while n > 0 {
                x = n%10;
                temp += (x * x);
                n = n/10;
                // println!("{x}, {temp}");
            }
            n = temp;
            temp = 0;
            if hs.contains(&n) {break;}
            hs.insert(n);
        }
        n == 1
    }
}

/*
*/

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let digits = |mut n| {
            std::iter::from_fn(move || {
                if n != 0 {
                    let r = n % 10;
                    n = n / 10;
                    Some(r)
                } else {
                    None
                }
            })
        };

        // i32::MAX = 2_147_483_647
        let max = 2 * 2 + 9 * 9 * 9;
        for _ in 0 ..= max {
            n = digits(n).map(|x| x * x).sum();
            if n == 1 {
                return true;
            }
        }
        return false;
    }
}