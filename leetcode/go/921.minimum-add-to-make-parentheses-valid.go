func minAddToMakeValid(s string) int {
    openCount, closedCount := 0, 0
    for _, c := range s {
        if c == '(' {
            openCount++
        } else if openCount > 0 {
            openCount--
        } else {
            closedCount++
        }
    }
    return openCount + closedCount
}

