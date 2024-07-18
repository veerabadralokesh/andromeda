impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let (m, k) = (m as i64, k as i64);
        if m * k > bloom_day.len() as i64 {return -1;}
        let bloom_day:Vec<i64> = bloom_day.iter().map(|&b| b as i64).collect();
        let get_bouquet_count = |wd: i64| -> i64 {
            let mut bouquet_count = 0;
            let mut required_flowers = k;
            for &b in bloom_day.iter() {
                if b > wd {
                    required_flowers = k;
                } else {
                    required_flowers -= 1;
                    if required_flowers == 0 {
                        bouquet_count += 1;
                        required_flowers = k;
                    }
                }
            }
            bouquet_count
        };

        let (mut l, mut r) = (*bloom_day.iter().min().unwrap(), *bloom_day.iter().max().unwrap());
        while l < r {
            let mid = l + (r - l)/2;
            if get_bouquet_count(mid) >= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        return l as _
    }
}

