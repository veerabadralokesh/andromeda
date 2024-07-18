impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut count = 0i32;
        let mut p1 = vec![0; 51];
        let mut p2 = vec![0; 51];
        let mut ans = Vec::new();
        for i in 0..a.len() {
            let (x, y) = (a[i] as usize, b[i] as usize);
            p1[x] += 1;
            p2[y] += 1;
            if p1[x] == p2[x] {
                count += 1;
            }
            if x != y && p1[y] == p2[y] {
                count += 1;
            }
            ans.push(count);
        }
        ans
    }
}
