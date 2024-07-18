use std::collections::HashMap;

static modulo: i64 = 1000000007;

impl Solution {

    fn rev(mut x: i32) -> i32 {
        let mut rx = 0;
        while x > 0 {
            rx = rx * 10 + x % 10;
            x = x/10;
        }
        rx
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let n = nums.len() as i32;
        for nx in nums {
            *map.entry((nx - Self::rev(nx))).or_insert(0i64) += 1;
        }
        let mut pairs = 0;
        for (k, v) in map.into_iter() {
            pairs += ((v * (v-1)) >> 1);
        }
        pairs %= modulo;
        pairs as i32
    }
}

/* */

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        // i + rj = j + ri
        // i + rk = k + ri
        // i - ri = k - rk = j - rj

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut ans: i32 = 0;
        let bound = 10i32.pow(9) + 7;

        for n in nums.iter() {
            let diff = n - rev(n);

            if let Some(cnt) = map.get_mut(&diff) {
                ans = (ans + *cnt) % bound;
                *cnt += 1;
            } else {
                map.insert(diff, 1i32);
            }
        }

        ans
    }
}

fn rev(n: &i32) -> i32 {
    let mut r = 0;
    let mut num = *n;
    while num > 0 {
        r = (r * 10 + num % 10);
        num /= 10;
    }

    r
}
