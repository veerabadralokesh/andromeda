impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn addNums(x: i32, n: i32, ans: &mut Vec<i32>) {
            ans.push(x);
            let y = x * 10;
            for i in y..y+10 {
                if i > n {
                    break;
                }
                addNums(i, n, ans);
            }
        }
        let mut ans = Vec::with_capacity(n as usize + 1);
        for i in 1..10 {
            if i > n {
                break;
            }
            addNums(i, n, &mut ans);
        }
        ans
    }
}

