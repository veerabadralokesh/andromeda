func findTheLongestSubstring(s string) int {
    prefixIdx := make(map[int]int)
    prefixIdx[0] = -1

    mask := 0
    ans := 0
    for i, r := range s {
        if strings.ContainsRune("aeiou", r) {
            mask ^= (1 << (r - 'a'))
        }
        if idx, ok := prefixIdx[mask]; ok {
            ans = max(ans, i - idx)
        } else {
            prefixIdx[mask] = i
        }
    }
    return ans
}

