func findLatestStep(arr []int, m int) int {
    if len(arr) == m {
        return m;
    }
    ranges := make([]int, len(arr) + 2)
    step := 0
    ans := -1
    start := 0
    end := 0
    for _, i := range arr {
        if ranges[i-1] == m || ranges [i+1] == m {
            ans = step
        }
        step++
        start = i - ranges[i-1]
        end = i + ranges[i+1]
        ranges[start] = end - start + 1
        ranges[end] = end - start + 1
    }
    return ans
}

