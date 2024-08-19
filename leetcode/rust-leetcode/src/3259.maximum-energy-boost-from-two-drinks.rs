use std::cmp::max;
impl Solution {
    pub fn max_energy_boost(energy_drink_a: Vec<i32>, energy_drink_b: Vec<i32>) -> i64 {
        let n = energy_drink_a.len();
        let energy_drink_a = energy_drink_a.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let energy_drink_b = energy_drink_b.iter().map(|&n| n as i64).collect::<Vec<_>>();
        let (mut dpa, mut dpb) = (vec![0; n], vec![0; n]);
        (dpa[0], dpb[0]) = (energy_drink_a[0], energy_drink_b[0]);
        (dpa[1], dpb[1]) = (
            dpa[0] + energy_drink_a[1], dpb[0] + energy_drink_b[1]
        );
        for i in 2..n {
            dpa[i] = energy_drink_a[i] + max(dpa[i-1], dpb[i-2]);
            dpb[i] = energy_drink_b[i] + max(dpb[i-1], dpa[i-2]);
        }
        // println!("{:?}", dpa);
        // println!("{:?}", dpb);
        max(dpa[n-1], dpb[n-1])
    }
}

