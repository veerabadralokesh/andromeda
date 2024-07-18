impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let nc = num.to_string().chars().collect::<Vec<_>>();
        let mut mx = "";
        for i in 2..nc.len() {
            if nc[i] == nc[i-1] && nc[i] == nc[i-2] && &num[i..i+1] > mx {
                mx = &num[i-2..i+1];
            }
        }
        mx.to_string()
    }
}

