impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut word_count = [0; 1024];
        word_count[0] = 1;
        let mut prefix = 0;
        let mut ans = 0;

        for b in word.into_bytes() {
            let word = 1 << ((b - b'a') as u8);
            prefix ^= word;
            ans += word_count[prefix];
            for i in 0..10 {
                ans += word_count[(prefix) ^ (1 << i)];
            }
            word_count[prefix] += 1;
        }
        ans
    }
}

/* */

impl Solution {

pub fn wonderful_substrings(word: String) -> i64 {

    let mut cnt = [0_i64; 1024];



    let (mut total, mut prefix) = (0, 0);



    cnt[0] = 1;



    for c in word.bytes() {

    prefix ^= (1 << (c - b'a')) as usize;



    total += cnt[prefix];



    for i in 0..10 {

        total += cnt[prefix ^ (1 << i)];

    }



    cnt[prefix] += 1;

    }
  
  

    total

}

}
