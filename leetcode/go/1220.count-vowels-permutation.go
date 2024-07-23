func countVowelPermutation(n int) int {
    kMod := 1_000_000_007
    dp := make([][5]int, n)
    dp[0] = [5]int{1, 1, 1, 1, 1}
    for i:=1; i<n; i++ {
        // a
        dp[i][0] = (dp[i-1][1] + dp[i-1][2] + dp[i-1][4]) % kMod
        // e
        dp[i][1] = (dp[i-1][0] + dp[i-1][2]) % kMod
        // i
        dp[i][2] = (dp[i-1][1] + dp[i-1][3]) % kMod
        // o
        dp[i][3] = dp[i-1][2];
        // u
        dp[i][4] = (dp[i-1][2] + dp[i-1][3]) % kMod
    }
    return (dp[n-1][0] + dp[n-1][1] + dp[n-1][2] + dp[n-1][3] + dp[n-1][4]) % kMod
}

