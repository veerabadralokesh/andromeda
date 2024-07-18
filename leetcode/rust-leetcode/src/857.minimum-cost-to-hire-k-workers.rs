use std::collections::BinaryHeap;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut ans = f64::MAX;
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        let mut workers = quality.into_iter().zip(wage).map(|(q, w)| ((w as f64)/(q as f64), q)).collect::<Vec<_>>();
        workers.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());

        let mut quality_sum = 0.0;
        for (wpq, q) in workers.into_iter() {
            heap.push(q);
            quality_sum += (q as f64);
            if heap.len() > k {
                quality_sum -= (heap.pop().unwrap() as f64);
            }
            if heap.len() == k {
                ans = ans.min((quality_sum as f64) * wpq);
            }
        }
        ans
    }
}

/* */

use std::collections::BinaryHeap;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut ratios = (
            quality
            .iter()
            .enumerate()
            .map(|(i, &q)| (wage[i] as f64 / q as f64, q))
            .collect::<Vec<(f64, i32)>>()
        );
        ratios.sort_unstable_by(|(a, _), (b, _)| a.partial_cmp(&b).unwrap());
        let mut qsum = 0 as i32;
        let mut h = BinaryHeap::<i32>::new();
        (
            ratios
            .iter()
            .fold(
                f64::MAX,
                |res, &(r, q)| {
                    h.push(q);
                    qsum += q;
                    if (h.len() > k) {qsum -= h.pop().unwrap();}
                    let min = qsum as f64 * r;
                    if (h.len() == k) {res.min(qsum as f64 * r)}
                    else {res}
                }
            )
        ) as _ 
    }
}
