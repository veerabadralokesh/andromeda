impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut ans = vec![0, 1];

        let (mut l, mut r) = (0f32, 1f32);

        loop {
            let m = (l + r)/2.0;
            ans[0] = 0;
            let mut count = 0;
            let mut j = 0;

            for i in 0..n {
                while j < n && arr[i] as f32 > (m * arr[j] as f32) {
                    j += 1;
                }
                count += ((n-j) as i32);
                if j == n {
                    break;
                }
                if ans[0] * arr[j] < ans[1] * arr[i] {
                    ans[0] = arr[i];
                    ans[1] = arr[j];
                }
            }

            if count < k {
                l = m;
            } else if count > k {
                r = m;
            } else {
                return ans;
            }
        }

        ans
    }
}

/* */

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut fracs = Vec::new();
        for i in 0..arr.len()-1 {
            for j in i+1..arr.len() {
                fracs.push((arr[i] as f32/arr[j] as f32, arr[i], arr[j]));
            }
        }
        fracs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let x = fracs[(k-1) as usize];
        vec![x.1, x.2]
    }
}

