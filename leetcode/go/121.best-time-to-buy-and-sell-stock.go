func maxProfit(prices []int) int {
    minp, maxp := math.MaxInt32, 0
    for _, p := range prices {
        if minp > p {
            minp = p
        } else if p - minp > maxp {
            maxp = p - minp
        }
    }
    return maxp
}

