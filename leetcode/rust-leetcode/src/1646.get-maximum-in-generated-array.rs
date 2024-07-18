impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 || n == 1 {return n;}
        let mut ans = 0i32;
        let n = n as usize;
        let mut arr = vec![0;(n+1)];
        arr[1]=1;
        for i in 2..n+1 {
            if i % 2 == 0 {
                arr[i] = arr[i/2];
            } else {
                arr[i] = arr[i/2] + arr[i/2 + 1];
            }
            ans = ans.max(arr[i]);
        }
        ans
    }
}
