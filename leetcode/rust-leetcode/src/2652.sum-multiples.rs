impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut sum:i32 = 0;
        let mut flag:bool = false;
        let (mut a, mut b, mut c) = (0i32, 0i32, 0i32);

        for i in 1..n+1 {
            if i == a + 3 {
               flag = true;
               a = i; 
            } 
            if i == b + 5 {
                flag = true;
                b = i;
            }
            if i == c + 7 {
                flag = true;
                c = i;
            }
            if flag {
                sum += i;
                flag = false;
            }
        }

        sum
    }
}

impl Solution2 {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut sum:i32 = 0;
        for i in 1..n+1 {
            if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
                sum += i;
            }
        }
        sum
    }
}