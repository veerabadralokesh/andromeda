use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn nth_super_ugly_number(mut n: i32, primes: Vec<i32>) -> i32 {
        if n == 1 { return 1 ;}
        let mut heap = BinaryHeap::new();
        let mut ans = 1;
        n -= 1;
        let mut primes = primes.iter().map(|p| *p as u64).collect::<Vec<u64>>();
        for p in &primes {heap.push(Reverse(*p));}
        while let Some(x) = heap.pop() {
            for p in &primes {
                heap.push(Reverse(x.0 * p));
            }
            while !heap.is_empty() && heap.peek().unwrap().0 == x.0 {
                heap.pop();
            }
            n-=1;
            if n == 0 {
                ans = x.0 as i32;
                break;
            }
        }
        ans
    }
}
