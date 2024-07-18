func minDeletions(s string) int {
    counts := make([]int, 26)
    for _, c := range s {
        counts[int(c - 'a')] += 1
    }
    slices.Sort(counts)
    m := make(map[int]bool)
    deletions := 0
    for _, c := range counts {
        if _, ok := m[c]; ok {
            for c > 0 {
                c--
                deletions++
                if _, ok := m[c]; !ok {
                    m[c] = true
                    break
                }
            }
        } else {
            m[c] = true
        }
    }
    return deletions
}

/* */

func minDeletions(s string) int {
    // we should count the frequence of all
    // character. And store it into an array.
    // then we sort the array in a asc order.
    // and use a map to store the frequence as
    // we iterate the array.

    alphaNumber := 26
    frequenceArray := make([]int, alphaNumber)

    charArray := []byte(s)

    for _, char := range charArray {
        frequenceArray[char-'a'] = frequenceArray[char-'a'] + 1
    }

    frequenceMap := map[int]bool{}

    var res int
    for i := 0; i < alphaNumber; i ++ {
        frequence := frequenceArray[i]
        for frequence > 0 && frequenceMap[frequence] {
            frequence --
            res ++
        }
        frequenceMap[frequence] = true
    }

    return res
}

