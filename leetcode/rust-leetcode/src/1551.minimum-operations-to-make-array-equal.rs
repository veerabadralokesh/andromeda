impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        // if n%2 == 0 {
        //     (n*n)/4
        // } else {
        //     (n*n-1)/4
        // }
        (n*n)/4
    }
}

/* */

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        // 1 ... 2n-1
        // 1 3 5 ... 2n-5 2n-3 2n-1
        //
        
        // n times on half. 1+n .. n-1
        // n-1 times on half. n .. n
        // n-1 times on half. n 3+n 5+n .. n-4 n-2 n
        // n-1, n-3, n-5
        // n, n, n ... n, n, n
        (n-(n/2))*(n/2)
    }
}

