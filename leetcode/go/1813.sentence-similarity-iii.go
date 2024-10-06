func areSentencesSimilar(sentence1 string, sentence2 string) bool {
    if len(sentence1) == len(sentence2) {
        return sentence1 == sentence2
    }
    
    words1 := strings.Split(sentence1, " ")
    words2 := strings.Split(sentence2, " ")

    m, n := len(words1), len(words2)

    if m > n {
        return areSentencesSimilar(sentence2, sentence1)
    }

    i := 0

    for i < m && words1[i] == words2[i] {
        i++
    }
    for i < m && words1[i] == words2[i + n - m] {
        i++
    }

    return i == m
}

