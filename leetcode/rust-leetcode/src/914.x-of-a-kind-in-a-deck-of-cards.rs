impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for &d in deck.iter() {
            *map.entry(d).or_insert(0) += 1;
        }
        let mut vals = map.values().cloned().collect::<Vec<_>>();
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {return a}
            gcd(b, a%b)
        }
        let mut count = vals[0];
        for &c in vals.iter().skip(1) {
            count = gcd(c, count);
        }
        count > 1
    }
}

