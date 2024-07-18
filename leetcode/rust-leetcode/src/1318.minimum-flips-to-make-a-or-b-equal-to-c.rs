impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        fn bin_arr (mut x: i32) -> ([bool; 32], usize) {
            let mut arr = [false; 32];
            let mut hb = 0usize;
            while x > 0 {
                if x & 1 == 1 {
                    arr[31-hb] = true;
                }
                hb += 1;
                x >>= 1;
            }
            (arr, hb)
        }
        let mut flips = 0;
        let (abin, ahb) = bin_arr(a);
        let (bbin, bhb) = bin_arr(b);
        let (cbin, chb) = bin_arr(c);
        let hb = ahb.max(bhb.max(chb));
        for i in (31-hb)..32 {
            if cbin[i] && !(abin[i] || bbin[i]) {
                flips += 1;
            }
            if !cbin[i] {
                if (abin[i]) {
                    flips += 1;
                }
                if (bbin[i]) {
                    flips += 1;
                }
            }
        }
        flips
    }
}
