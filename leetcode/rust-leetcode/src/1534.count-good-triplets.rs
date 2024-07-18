impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let l = arr.len();
        let mut ans = 0;
        for i in 0..l-2 {
            for j in i+1..l-1 {
                if (arr[i]-arr[j]).abs() > a {
                    continue;
                }
                for k in j+1..l {
                    if (arr[j]-arr[k]).abs() > b || (arr[k]-arr[i]).abs() > c {
                        continue;
                    } else {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

