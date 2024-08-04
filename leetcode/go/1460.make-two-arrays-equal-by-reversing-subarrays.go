func canBeEqual(target []int, arr []int) bool {
    if len(target) != len(arr) {
        return false
    }
    counts := make([]int, 1001)
    for _, n := range arr {
        counts[n]++
    }
    for _, n := range target {
        if counts[n] == 0 {
            return false
        }
        counts[n]--
    }
    return true
}

