// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
		let (mut l, mut r) = (1, n as i64);
        while l < r {
            let m = (l + r) / 2;
            if self.isBadVersion(m as i32) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
}

