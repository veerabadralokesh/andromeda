func smallestDistancePair(nums []int, k int) int {
    slices.Sort(nums)
    n := len(nums)
    valid_distance := func(d int) bool {
        count := 0
        i, j := 0, 0
        for i < n || j < n {
            for j < n && nums[j] - nums[i] <= d {
                j++
            }
            count += j - i - 1
            i++
        }
        return count >= k
    }
    start, end := 0, nums[n-1]-nums[0]
    for start < end {
        mid := start + (end-start)/2
        if valid_distance(mid) {
            end = mid
        } else {
            start = mid + 1
        }
    }
    return start
}

