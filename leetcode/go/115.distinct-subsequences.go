func numDistinct(s string, t string) int {
    m, n := len(s), len(t)
    dp := make([]int, n + 1)
    dp[0] = 1
    for i:=1; i<m+1; i++ {
        for j:=n; j>0; j-- {
            if s[i-1] == t[j-1] {
                dp[j] += dp[j-1]
            }
        }
    }
    return dp[n]
}

