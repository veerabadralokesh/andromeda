impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut mx = 0i64;
        let mut mi = 0i64;
        let piles = piles.into_iter().map(|p| p as i64).collect::<Vec<_>>();
        let h = h as i64;
        fn div_ceil(y: i64, x:i64) -> i64 {
            let mut rem = y/x;
            if x * rem < y {
                rem += 1;
            }
            rem
        }
        for &p in &piles {
            mx = mx.max(p);
            mi += p as i64;
        }
        mi = div_ceil(mi, h);
        let mut mi = mi as i64;
        while mi < mx {
            let speed = (mi+mx)/2;
            let mut sum = 0;
            for &p in &piles {
                sum += div_ceil(p, speed);
            }
            if sum <= h {
                mx = speed;
            } else {
                mi = speed + 1;
            }
        }
        mi as i32
    }
}
