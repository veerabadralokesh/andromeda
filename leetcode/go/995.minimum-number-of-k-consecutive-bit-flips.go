func minKBitFlips(nums []int, k int) int {
    ans := 0
    flipped := 0
    size := len(nums)
    for i, n := range nums {
        if i >= k && nums[i-k] == 2 {
            flipped--
        }
        if flipped & 1 == n {
            if i + k > size {
                return -1
            }
            flipped++
            ans++
            nums[i] = 2
        }
    }
    return ans
}

