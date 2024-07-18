impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        if beans.len() == 1 { return 0; }
        beans.sort();
        let beans = beans.iter().map(|&b| b as i64).collect::<Vec<_>>();
        let total = beans.iter().sum::<i64>();
        let mut ans = total;
        let l = beans.len() as i64;
        for i in 0..l {
            ans = ans.min(total - beans[i as usize] * (l - i));
        }
        ans
    }
}

