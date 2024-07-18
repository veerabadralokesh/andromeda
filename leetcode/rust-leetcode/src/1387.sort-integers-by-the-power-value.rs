use std::collections::HashMap;
impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut collatz:HashMap<i32,i32> = HashMap::new();
        fn collatz_steps(map: &mut HashMap<i32,i32>, n: i32) -> i32 {
            if n == 1 {return 0;}
            if n == 2 {return 1;}
            if map.contains_key(&n) {
                return *map.get(&n).unwrap();
            }
            let mut ans = 1i32;
            if n % 2 == 0 {
                ans = 1 + collatz_steps(map, n/2);
            } else {
                ans = 1 + collatz_steps(map, 3*n + 1);
            }
            map.insert(n, ans);
            ans
        }
        collatz.insert(1, 0);
        collatz.insert(2, 1);
        let n = (hi-lo+1) as usize;
        let mut steps = vec![[0i32, 0i32]; n];
        for (i,n) in (lo..=hi).enumerate() {
            steps[i][1] = n;
            if !collatz.contains_key(&n) {
                steps[i][0] = collatz_steps(&mut collatz, n);
            } else {
                steps[i][0] = *collatz.get(&n).unwrap();
            }
        }
        steps.sort();
        // println!("{:?}", steps);
        steps[(k-1) as usize][1]
    }
}
