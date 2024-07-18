func minDeletionSize(strs []string) int {
    m, n := len(strs), len(strs[0])
    ans := 0
    for j := range n {
        for i := 1; i < m ; i++ {
            if strs[i][j] < strs[i-1][j] {
                ans++
                break
            }
        }
    }
    return ans
}

