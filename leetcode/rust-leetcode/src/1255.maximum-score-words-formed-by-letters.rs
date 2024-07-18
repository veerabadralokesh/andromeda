impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut counts = [0; 26];
        for i in 0..letters.len() {
            let idx = (letters[i] as u8 - b'a') as usize;
            counts[idx] += 1;
        }
        let word_vec = words.to_vec().into_iter().map(|w| w.into_bytes().into_iter().map(|b| (b - b'a') as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
        let word_scores = word_vec.to_vec().into_iter().map(|w| w.into_iter().fold(0, |acc, idx| acc + score[idx])).collect::<Vec<_>>();

        fn word_score(i: usize, counts: &mut [i32], word_scores: &Vec<i32>, word_vec: &Vec<Vec<usize>>) -> i32 {
            let mut can_use_word = true;
            for &idx in word_vec[i].iter() {
                if counts[idx] == 0 {
                    can_use_word = false;
                }
                counts[idx] -= 1;
            }
            if can_use_word {word_scores[i]} else {-1}
        }
        fn remove_word(i: usize, counts: &mut [i32], word_vec: &Vec<Vec<usize>>) {
            for &idx in word_vec[i].iter() {
                counts[idx] += 1;
            }
        };

        fn backtrack(i: usize, l: usize, word_vec: &Vec<Vec<usize>>, word_scores: &Vec<i32>, counts: &mut [i32]) -> i32 {
            let mut ans = 0;
            for j in i..l {
                let ws = word_score(j, counts, word_scores, word_vec);
                if ws > 0 {
                    ans = ans.max(ws + backtrack(j+1, l, word_vec, word_scores, counts));
                }
                remove_word(j, counts, word_vec);
                // ans = ans.max(backtrack(j+1, l, word_vec, word_scores, counts));
            }
            ans
        };
        backtrack(0, words.len(), &word_vec, &word_scores, &mut counts)
    }
}

