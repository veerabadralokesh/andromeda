func minSwaps(s string) int {
    open, swapPosition, swaps := 0, -1, 0
    for i, c := range s {
        if c == '[' {
            open++
        } else if open > 0 {
            open--
        } else {
            if swapPosition < 0 {
                swapPosition = i
                swaps++
            } else {
                swapPosition = -1
            }
        }
    }
    return swaps
}

