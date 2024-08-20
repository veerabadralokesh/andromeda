func stoneGameII(piles []int) int {
    n := len(piles)
    memo := make([][]int, n)
    for i := range n {
        memo[i] = make([]int, n)
    }
    suffix := make([]int, n)
    copy(suffix, piles)
    for i:=n-2; i > -1; i-- {
        suffix[i] += suffix[i+1]
    }
    return dp(&memo, &suffix, 0, 1)
}

func dp(memo *[][]int, suffix *[]int, i int, m int) int {
    if i + 2 * m >= len((*suffix)) {
        return (*suffix)[i]
    }
    if (*memo)[i][m] == 0 {
        bob := (*suffix)[i]
        for X:=1; X < 2*m + 1; X++ {
            bob = min(bob, dp(memo, suffix, i+X, max(X, m)))
        }
        (*memo)[i][m] = (*suffix)[i] - bob
    }
    return (*memo)[i][m]
}

