impl Solution {
    pub fn fair_candy_swap(mut alice_sizes: Vec<i32>, mut bob_sizes: Vec<i32>) -> Vec<i32> {
        alice_sizes.sort();
        bob_sizes.sort();
        let mut asum = alice_sizes.iter().sum::<i32>();
        let mut bsum = bob_sizes.iter().sum::<i32>();

        let (a, b, asum, bsum, diff, flip) = if asum > bsum {
            (alice_sizes, bob_sizes, asum, bsum, asum-bsum, false)
        } else {
            (bob_sizes, alice_sizes, bsum, asum, bsum-asum, true)
        };

        for &n1 in a.iter() {
            let n2 = (2*n1-diff)/2;
            match b.binary_search(&n2) {
                Ok(idx) => {
                    if flip {
                        return vec![n2, n1]
                    } else {
                        return vec![n1, n2]
                    }
                }
                Err(_) => {}
            }
        }
        vec![0, 0]
    }
}

