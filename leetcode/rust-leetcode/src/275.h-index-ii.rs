impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut l, mut r, mut m, n) = (0, citations.len(), 0, citations.len());
        while l < r {
            m = l + (r - l)/2;
            if (citations[m] as usize) >= (n - m) {
                r = m;
            } else {
                l = m+1;
            }
        }
        (n-l) as _
    }
}

