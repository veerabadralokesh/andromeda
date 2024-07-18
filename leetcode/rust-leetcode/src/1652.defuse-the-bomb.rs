impl Solution {
    pub fn decrypt(mut code: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut decrypted = vec![0; code.len()];
        if k == 0 {
            return decrypted;
        }
        decrypted.append(&mut vec![0;code.len()]);
        let mut k_is_neg = false;
        if k < 0 {
            k_is_neg = true;
            k = -k;
            code.reverse();
        }
        let k = k as usize;
        decrypted[0] = code[0];
        for i in 1..code.len() {
            decrypted[i] = decrypted[i-1] + code[i];
        }
        for i in 0..code.len().min(k) {
            decrypted[code.len()+i] = decrypted[code.len()+i-1] + code[i];
        }
        for i in k..k+code.len() {
            code[i-k] = decrypted[i] - decrypted[i-k];
        }
        if k_is_neg {
            code.reverse();
        }
        code
    }
}

