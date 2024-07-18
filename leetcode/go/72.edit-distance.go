func minDistance(word1 string, word2 string) int {
    m, n := len(word1), len(word2)
    dp := make([][]int, m+1)
    for i := range m+1 {
        dp[i] = make([]int, n+1)
        dp[i][0] = i
    }
    for j := range n+1 {
        dp[0][j] = j
    }
    for i:=0; i < m ; i++ {
        for j:=0; j < n; j++ {
            if word1[i] == word2[j] {
                dp[i+1][j+1] = dp[i][j]
            } else {
                dp[i+1][j+1] = 1 + Min(dp[i][j], Min(dp[i+1][j], dp[i][j+1]))
            }
        }
    }
    return dp[m][n]
}

func Min(a int, b int) int {
    if a < b {return a}
    return b
}

