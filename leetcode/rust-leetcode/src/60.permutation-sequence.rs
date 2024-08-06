static facts: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;
        fn permute(n: usize, k: usize) -> Vec<usize> {
            let mut arr = vec![0; n];
            for i in 1..=n {
                arr[i-1] = i;
            }
            if n == 1 || k == 0 {
                return arr;
            }
            let fac_n_1 = facts[n-1];
            let (mut si, pi) = (k/fac_n_1, k%fac_n_1);
            while si > 0 {
                arr.swap(si, si-1);
                si -= 1;
            }
            let sub_arr = permute(n-1, pi);
            let arrc = arr.to_vec();
            for i in 1..arr.len() {
                arr[i] = arrc[sub_arr[i-1]];
            }
            // println!("{fac_n_1} {si} {pi} {:?} {:?}", arr, sub_arr);
            arr
        }
        let arr = permute(n, k-1);
        arr.into_iter().map(|n| (n as u8 + b'0') as char).collect()
    }
}

