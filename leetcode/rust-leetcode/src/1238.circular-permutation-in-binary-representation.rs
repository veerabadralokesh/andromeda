impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        fn gray_codes(n: i32, v: &mut Vec<i32>, idx: &mut usize, start: i32) {
            if n == 1 {
                v.push(0);
                v.push(1);
                if start == 0 {
                    *idx = 0;
                } else if start == 1 {
                    *idx = 1;
                }
                return;
            }
            let n1 = n-1;
            gray_codes(n1, v, idx, start);
            let mut j = v.len();
            for i in (0..v.len()).rev() {
                v.push(v[i] | (1 << n1));
                if v[j] == start {
                    *idx = j;
                }
                j += 1;
            }
        }
        let mut v = Vec::with_capacity((2_usize.pow(n as u32)));
        let mut idx = 0;
        gray_codes(n, &mut v, &mut idx, start);
        v.rotate_left(idx);
        v
    }
}

/* */

// LEARN
// gray code

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut gray_cycle: Vec<_> = (0..(1 << n)).map(|i| i ^ (i >> 1)).collect();
        let idx = gray_cycle.iter().position(|&x| x == start).unwrap();
        gray_cycle.rotate_left(idx);
        gray_cycle
    }
}

/* */

// gray code

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        (0..(1 << n)).map(|i| start ^ i ^ (i >> 1)).collect()
    }
}


