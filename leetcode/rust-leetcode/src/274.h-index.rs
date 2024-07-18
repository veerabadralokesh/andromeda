impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        citations.reverse();
        let mut hindex:i32 = 0;
        let mut papers:i32 = 0;
        for c in citations {
            papers += 1;
            if papers <= c {
                hindex = papers;
            } else {
                break;
            }
        }
        hindex
    }
}
