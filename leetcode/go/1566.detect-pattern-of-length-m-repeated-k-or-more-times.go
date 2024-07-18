func containsPattern(arr []int, m int, k int) bool {
    count := 0
    for i := m ; i < len(arr) ; i++ {
        if arr[i] == arr[i-m] {
            count++
        } else {
            count = 0
        }
        if count == m * k - m {
            return true
        }
    }
    return false
}

