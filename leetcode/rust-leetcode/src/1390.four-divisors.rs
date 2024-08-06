use std::collections::HashMap;
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let sqrt = |x: i32| -> i32 {
            (f64::sqrt(x as f64)+1.0) as i32
        };
        let mut map = HashMap::with_capacity(nums.len());
        let mut ans = 0;
        for n in nums {
            match map.get(&n) {
                Some(&(count, sum)) => {
                    if count == 4 {
                        ans += sum;
                    }
                },
                None => {
                    let mut count = 2;
                    let mut sum = 1 + n;
                    let sqrtn = sqrt(n);

                    for i in 2..sqrtn {
                        if n % i == 0 {
                            // println!("{n} {i}");
                            count += 1;
                            sum += i;
                            if n / i != i {
                                count += 1;
                                sum += n/i;
                            }
                        }
                    }

                    // println!("{count} {n}");

                    if count == 4 {
                        ans += sum;
                    }
                    map.insert(n, (count, sum));
                }
            }
        }
        ans
    }
}

