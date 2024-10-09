func minIncrementOperations(nums []int, k int) int64 {
    var dp, p1, p2, p3 int64 = 0, 0, 0, 0
    var k64 int64 = int64(k)
    for _, n := range nums {
        dp = min(p1, min(p2, p3)) + max(0, k64-int64(n))
        p3 = p2
        p2 = p1
        p1 = dp
    }
    return min(p1, min(p2, p3))
}

