func minPatches(nums []int, n int) int {
    missing, ans, i, l := 1, 0, 0, len(nums)
    for missing <= n {
        if i < l && nums[i] <= missing {
            missing += nums[i]
            i++
        } else {
            missing <<= 1
            ans++
        }
    }
    return ans
}

