func findShortestSubArray(nums []int) int {
    left := make(map[int]int)
    right := make(map[int]int)
    counts := make(map[int]int)
    for i, n := range nums {
        if _, ok := left[n]; !ok {
            left[n] = i
        }
        right[n] = i
        counts[n]++
    }
    ans := len(nums)
    max_count := 0
    for k, v := range counts {
        if v > max_count {
            max_count = v
            ans = right[k] - left[k] + 1
        } else if v == max_count {
            ans = Min(ans, right[k]-left[k]+1)
        }
    }
    return ans
}

func Min(x int, y int) int {
    if x < y {return x}
    return y
}

