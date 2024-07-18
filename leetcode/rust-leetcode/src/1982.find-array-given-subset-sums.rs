use std::collections::HashMap;
impl Solution {
    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        sums.sort();
        let ns = sums.len();
        if sums[0] == sums[(ns-1) as usize] {
            return vec![sums[0]/n; n as usize];
        }
        fn recurse(sums: Vec<i32>) -> Vec<i32> {
            if sums.len() <= 1 {return Vec::new();}
            let mut counts = sums
                            .iter()
                            .copied()
                            .fold(HashMap::new(), |mut map, val|{
                                map.entry(val)
                                    .and_modify(|frq|*frq+=1)
                                    .or_insert(1);
                                map
                            });
            let num = sums[1] - sums[0];
            let mut sum_negative_num = Vec::new();//with_capacity(sums.len()/2);
            let mut sum_positive_num = Vec::new();//with_capacity(sums.len()/2);
            let mut is_number_positive = true;
            for s in sums {
                if *counts.get(&s).unwrap() <= 0 {
                    continue;
                }
                *counts.entry(s).or_insert(0) -= 1;
                *counts.entry((s+num)).or_insert(0) -= 1;
                // if !is_number_positive {
                sum_negative_num.push(s);
                // }
                sum_positive_num.push(s+num);
                if s + num == 0 {
                    is_number_positive = false;
                }
            }
            let mut ans = if is_number_positive {
                recurse(sum_negative_num)
            } else {
                recurse(sum_positive_num)
            };
            if is_number_positive {ans.push(num);} else {ans.push(-num);};
            ans
        }
        recurse(sums)
    }
}

/* */


impl Solution {
    pub fn recover_array(n: i32, mut sums: Vec<i32>) -> Vec<i32> {
        sums.sort_unstable();
        let mut sums = &mut sums[..];

        let mut zero_shift = 0;
        let mut rez = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let m = sums.len();
            let diff = sums[1] - sums[0];

            let (mut j, mut k) = (0, 0);
            for i in 0..m {
                if k < j && sums[i] == sums[k] {
                    k += 1;
                    continue;
                }
                sums[j] = sums[i] + diff;
                j += 1;
            }

            match sums[..m >> 1].binary_search(&zero_shift) {
                Ok(_) => rez.push(-diff),
                _ => {
                    zero_shift += diff;
                    rez.push(diff)
                }
            }
            sums = &mut sums[..m >> 1];
        }

        rez
    }
}
