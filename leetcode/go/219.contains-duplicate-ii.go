func containsNearbyDuplicate(nums []int, k int) bool {
    m := make(map[int]int, len(nums))
    for i, n := range nums {
        if val, ok := m[n]; ok && i - val <= k {
            return true
        }
        m[n] = i
    }
    return false
}

