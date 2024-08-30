func longestPrefix(s string) string {
    prefixLen := calcPrefixLen(s)
    return s[0:prefixLen[len(prefixLen)-1]]
}

// calculating KMP prefix arr
func calcPrefixLen(pattern string) []int {
    patternLen := len(pattern)

    arr := make([]int, patternLen+1)
    arr[0] = -1

    prefixLen := 0
    i := 1

    for i < patternLen {
        if pattern[i] == pattern[prefixLen] {
            prefixLen++
            i++
            arr[i] = prefixLen
        } else if prefixLen > 0 {
            prefixLen = arr[prefixLen]
        } else {
            i++
            arr[i] = 0
        }
    }
    return arr
}

