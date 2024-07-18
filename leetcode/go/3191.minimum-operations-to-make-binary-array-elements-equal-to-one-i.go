func minOperations(nums []int) int {
    ans, flipped, size := 0, 0, len(nums)
    for i, n := range nums {
        if i > 2 && nums[i-3] == 2 {
            flipped--
        }
        if flipped & 1 == n {
            if i + 3 > size {
                return -1
            }
            nums[i] = 2
            ans++
            flipped++
        }
    }
    return ans
}

