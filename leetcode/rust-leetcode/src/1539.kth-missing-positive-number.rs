impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut ans = 1;
        let mut i = 0;
        while i < arr.len() {
            if arr[i] != ans {
                k-=1;
            } else {
                i += 1;
            }
            if k == 0 {
                break;
            }
            ans += 1;
        }
        for _ in 0..k-1 {
            ans += 1;
        }
        ans
    }
}

