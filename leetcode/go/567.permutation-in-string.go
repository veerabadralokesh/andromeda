func checkInclusion(s1 string, s2 string) bool {
    if len(s1) > len(s2) {
        return false
    }

    s1Counts := make([]int, 26)
    for _, c := range s1 {
        s1Counts[c-'a']++
    }

    s2Counts := make([]int, 26)
    l := len(s1)

    for i := 0; i < l; i++ {
        s2Counts[s2[i]-'a']++
    }

    for i := l; i < len(s2); i++ {
        if equal(s1Counts, s2Counts) {
            return true
        }

        s2Counts[s2[i]-'a']++
        s2Counts[s2[i-l]-'a']--
    }

    if equal(s1Counts, s2Counts) {
        return true
    }

    return false
}

func equal(a, b []int) bool {
    for i := 0; i < len(a); i++ {
        if a[i] != b[i] {
            return false
        }
    }
    return true
}


