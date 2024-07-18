impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut cover = [0; 52];
        for range in ranges {
            cover[range[0] as usize] += 1;
            cover[range[1] as usize + 1] -= 1;
        }
        let mut count = 0;
        for i in 0..52 {
            count += cover[i as usize];
            if left <= i && i <= right && count <= 0 {
                return false
            }
        }
        true
    }
}
