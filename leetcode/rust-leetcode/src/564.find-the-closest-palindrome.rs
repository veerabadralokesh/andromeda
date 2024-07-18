impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        // 1234, 999,1000, 12932, 99800, 12120
        fn close_palindrome(n: i64, nl: usize, add: i64) -> i64 {
            let nlby2 = nl / 2;
            // let ns = n.to_string().chars().collect::<Vec<_>>();
            // let mut first_half = (&ns[..(nlby2 + (nl & 1))].iter()
            //     .collect::<String>().parse::<i64>().unwrap() + add);
            let mut first_half = n;
            for _ in (0..(nl - nlby2 - (nl & 1))) {
                first_half /= 10;
            }
            first_half += add;
            // println!("{:?} {n} {nlby2} {nl} {add}", first_half);
            let mut second_half = first_half;
            for _ in 0..((nl & 1)) {
                second_half /= 10;
            }
            // println!("{:?}", second_half);
            while second_half > 0 {
                first_half = first_half * 10 + second_half % 10;
                second_half /= 10;
            }
            // println!("{:?}", first_half);
            first_half
        }
        let nl = n.len();
        let n = n.parse::<i64>().unwrap();

        let mut palindromes = vec![
            close_palindrome(n, nl, 0),
            close_palindrome(n, nl, 1),
            close_palindrome(n, nl,-1),
            close_palindrome(
                (0..nl).map(|i| '9').collect::<String>().parse::<i64>().unwrap()+1, nl+1, 0
            ),
        ];
        if nl > 1 {
            palindromes.push(close_palindrome(
                (0..nl-1).map(|i| '9').collect::<String>().parse::<i64>().unwrap(), nl-1, 0
            ));
        }
        // palindromes.sort();
        // println!("{:?}", palindromes);
        // palindromes[0].to_string()
        let mut ans = None;
        for &p in palindromes.iter() {
            if p == n {
                continue;
            }
            if ans.is_none() {
                ans = Some(p);
                continue;
            }
            let a = ans.unwrap();
            if (a - n).abs() > (p - n).abs() || ((a - n).abs() == (p - n).abs() && n > p) {
                ans = Some(p);
            }
        }
        ans.unwrap().to_string()
    }
}

