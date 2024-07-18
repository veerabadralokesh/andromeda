impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        if 2 * k > n * (n+1) {
            return Vec::new();
        }
        let mut combos = Vec::new();
        let mut combo = Vec::new();
        fn gen_combos(combos: &mut Vec<Vec<i32>>, combo: &mut Vec<i32>, start: i32, k: i32, n:i32, sum: i32) {
            if k == 0 {
                if sum == n {
                    combos.push(combo.to_vec());
                }
                return;
            }
            if start == 10 || start > n {
                return;
            }
            gen_combos(combos, combo, start+1, k, n, sum);
            combo.push(start);
            gen_combos(combos, combo, start+1, k-1, n, sum+start);
            combo.pop();
        }
        gen_combos(&mut combos, &mut combo, 1, k, n, 0);
        combos
    }
}
