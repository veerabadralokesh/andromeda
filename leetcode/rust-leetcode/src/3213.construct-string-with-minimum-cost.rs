impl Solution {
    pub fn minimum_cost(target: String, words: Vec<String>, costs: Vec<i32>) -> i32 {
        let mut t = Trie::new();
        for (i,word) in words.iter().enumerate() {
            t.insert(word.clone(), costs[i]);
        }
        let n = target.len();
        let mut dp = vec![1000000000; n+1];
        dp[n] = 0;
        let tb = target.into_bytes();
        for i in (0..n).rev() {
            let mut trie = &t;
            for j in i..n {
                let next = unsafe { trie.next.get_unchecked((tb[j] - b'a') as usize) };
                if let Some(next) = next {
                    let next = next.as_ref();
                    if next.cost != 0 {
                        // println!("{:?} {:?} {j} {}", next.cost, dp[j+1], dp[i]);
                        // println!("{:?}", dp);
                        dp[i] = dp[i].min(next.cost + dp[j+1]);
                    }
                    trie = &next;
                } else {
                    break;
                }
            }
        }
        // println!("{:?}", dp);
        if dp[0] >= 1000000000 {-1} else {dp[0]}
    }
}

#[derive(Default)]
#[derive(Debug)]
#[derive(Clone)]
struct Trie {
    end: bool,
    cost: i32,
    next: [Option<Box<Trie>>; 26],
}

impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Default::default()
    }
    
    #[inline(always)]
    fn insert(&mut self, word: String, cost: i32) {
        self._insert(word.as_bytes(), cost);
    }

    fn _insert(&mut self, word: &[u8], cost: i32) {
        match word.split_first() {
            Some((chr,rest)) => {
                let next = unsafe { self.next.get_unchecked_mut((chr - b'a') as usize) };
                next.get_or_insert_with(Default::default)._insert(rest, cost)
            },
            None => { self.end = true; self.cost = if self.cost == 0 {cost} else {cost.min(self.cost)};},
        }
    }
}

