
func validNum(a byte, b byte) bool {
    if b != 'b' {
        return a == '1' || a == '2' && b < '7'
    }
    return a != '0'
}

func numDecodings(s string) int {
    n := len(s)
    dp := make([]int, n+1)
    dp[n] = 1

    if validNum(s[n-1], 'b') {
        dp[n-1] = 1
    }
    for i := n-2; i > -1; i-- {
        if validNum(s[i], 'b') {
            dp[i] += dp[i+1]
        }
        if validNum(s[i], s[i+1]) {
            dp[i] += dp[i+2]
        }
    }
    return dp[0]
}

