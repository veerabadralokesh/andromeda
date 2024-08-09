func minHeightShelves(books [][]int, shelfWidth int) int {
    l := len(books)
    dp := make([]int, l+1)
    maxh, sumt := 0, 0
    for i:=0; i<l; i++ {
        dp[i+1] = math.MaxInt32
        maxh, sumt = 0, 0
        for j:=i; j > -1; j-- {
            sumt += books[j][0]
            if sumt > shelfWidth {
                break
            }
            maxh = max(maxh, books[j][1])
            dp[i+1] = min(dp[i+1], dp[j] + maxh)
        }
    }
    return dp[l]
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

