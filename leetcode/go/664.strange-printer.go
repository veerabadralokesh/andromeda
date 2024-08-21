func strangePrinter(s string) int {
    n := len(s)
    if n == 0 {
        return 0
    }
    dp := make([][]int, n)
    for i := range n {
        dp[i] = make([]int, n)
        for j:=0; j < n; j++ {
            dp[i][j] = n;
        }
        dp[i][i] = 1;
    }
    for j:=0; j < n; j++ {
        for i:=j; i>-1;i-- {
            for k:=i; k < j; k++ {
                dp[i][j] = min(dp[i][j], dp[i][k] + dp[k+1][j] - bool2Int(s[k] == s[j]))
            }
        }
    }
    return dp[0][n-1]
}

func bool2Int(x bool) int {
    if x {
        return 1
    }
    return 0
}

/* */

func strangePrinter(s string) int {
    n := len(s)
    if n == 0 {
        return 0
    }
    dp := make([][]int, n)
    for i := range n {
        dp[i] = make([]int, n)
    }
    return helper(&s, 0, n-1, &dp)
}

func helper(s *string, i int, j int, dp *[][]int) int {
    if i > j {
        return 0
    }
    if (*dp)[i][j] == 0 {
        (*dp)[i][j] = 1 + helper(s, i+1, j, dp)

        for k:=i+1; k<=j; k++ {
            if (*s)[i] == (*s)[k] {
                (*dp)[i][j] = min((*dp)[i][j], helper(s, i, k-1, dp) + helper(s, k+1, j, dp))
            }
        }
    }
    return (*dp)[i][j]
}


