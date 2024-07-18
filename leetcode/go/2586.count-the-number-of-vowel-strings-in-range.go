func vowelStrings(words []string, left int, right int) int {
    count := 0
    for i:=left ; i < right+1; i++ {
        if isVowelString(words[i]) {
            count++
        }
    }
    return count
}

func isVowelString(word string) bool {
    f, l := word[0], word[len(word)-1]
    if ((f == 'a' || f == 'e' || f == 'i' || f == 'o' || f == 'u') && (l == 'a' || l == 'e' || l == 'i' || l == 'o' || l == 'u')) {
        return true
    }
    return false
}

