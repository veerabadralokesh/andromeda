impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1.len() == sentence2.len() {
            return sentence1 == sentence2;
        }
        let words1 = sentence1.split_whitespace().collect::<Vec<_>>();
        let words2 = sentence2.split_whitespace().collect::<Vec<_>>();
        let (m, n) = (words1.len(), words2.len());
        if m > n {
            return Self::are_sentences_similar(sentence2, sentence1);
        }
        let mut i = 0;

        while i < m && words1[i] == words2[i] {
            i += 1;
        }
        while i < m && words1[i] == words2[i + n - m] {
            i += 1;
        }
        
        i == m
    }
}

