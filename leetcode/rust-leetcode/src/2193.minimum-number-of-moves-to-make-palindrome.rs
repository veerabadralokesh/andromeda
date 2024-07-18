impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        fn count_swaps(sc: &mut [char]) -> i32 {
            if sc.len() < 2 {
                return 0;
            }
            let mut swaps = 0;
            let (mut i, l, mut j) = (1, sc.len()-1, 0);
            if sc[0] != sc[l] {
                while sc[i] != sc[l] && i < l {
                    i += 1;
                }
                j = l - 1;
                while sc[j] != sc[0] && j > 0 {
                    j -= 1;
                }
                if i < l - j {
                    swaps = i;
                    // sc.swap(0, i);
                    for idx in (0..i).rev() {
                        sc.swap(idx, idx+1);
                    }
                } else {
                    swaps = l - j;
                    // sc.swap(j, l);
                    for idx in (j..l) {
                        sc.swap(idx, idx+1);
                    }
                }
            }
            swaps as i32 + count_swaps(&mut sc[1..l])
        }
        let mut sc = s.chars().collect::<Vec<_>>();
        // let ans = count_swaps(&mut sc);
        // println!("{:?}", sc);
        // ans
        count_swaps(&mut sc)
    }
}

