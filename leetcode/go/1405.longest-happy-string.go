func longestDiverseString(a int, b int, c int) string {
    return helper(a, b, c, 'a', 'b', 'c')
}

func helper(a int, b int, c int, A rune, B rune, C rune) string {
    if a < b {
        return helper(b, a, c, B, A, C)
    }
    if b < c {
        return helper(a, c, b, A, C, B)
    }
    if b == 0 {
        return strings.Repeat(string(A), min(a, 2))
    }
    aCount := min(a, 2)
    bCount := 0
    if (a - aCount >= b) {
        bCount = 1
    }
    return strings.Repeat(string(A), aCount) + strings.Repeat(string(B), bCount) + helper(a-aCount, b-bCount, c, A, B, C)
}

