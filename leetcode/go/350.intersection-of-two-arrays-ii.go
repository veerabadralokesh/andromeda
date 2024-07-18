func intersect(nums1 []int, nums2 []int) []int {
    masks := make([]int, 1001)
    for _, n := range nums1 {
        masks[n]++
    }
    ans := make([]int, 0, 1000)
    for _, n := range nums2 {
        if masks[n] > 0 {
            ans = append(ans, n)
            masks[n]--
        }
    }
    return ans
}

