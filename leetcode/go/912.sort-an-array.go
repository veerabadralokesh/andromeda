func sortArray(nums []int) []int {
    counts := make([]int, 100001)
    for _, n := range nums {
        counts[50000+n]++
    }
    ans := make([]int, 0, len(nums))
    for i, count := range counts {
        for _ = range count {
            ans = append(ans, i-50000)
        }
    }
    return ans
}

