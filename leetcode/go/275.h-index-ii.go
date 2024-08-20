func hIndex(citations []int) int {
    n, l, r, m := len(citations), 0, len(citations), 0
    for l < r {
        m = (l + r)/2
        if citations[m] >= n - m {
            r = m
        } else {
            l = m+1
        }
    }
    return n - l
}

