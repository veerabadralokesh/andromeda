func uncommonFromSentences(s1 string, s2 string) []string {
    counts := make(map[string]int)
    for _, word := range strings.Fields(s1) {
        if _, ok := counts[word]; ok {
            counts[word]++
        } else {
            counts[word] = 1
        }
    }
    for _, word := range strings.Fields(s2) {
        if _, ok := counts[word]; ok {
            counts[word]++
        } else {
            counts[word] = 1
        }
    }
    ans := make([]string, 0, 100)
    for word, count := range counts {
        if count == 1 {
            ans = append(ans, word)
        }
    }
    return ans
}

