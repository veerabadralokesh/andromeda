func maxChunksToSorted(arr []int) int {
    sarr := make([]int, len(arr))
    copy(sarr, arr)
    slices.Sort(sarr)
    sum, sorted_sum, ans := 0, 0, 0
    for i := 0; i < len(arr) ; i++ {
        sum += arr[i]
        sorted_sum += sarr[i]
        if sum == sorted_sum {
            ans++
        }
    }
    return ans
}

