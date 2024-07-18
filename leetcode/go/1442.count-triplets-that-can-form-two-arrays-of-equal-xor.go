func countTriplets(arr []int) int {
    xorarr := make([]int, len(arr)+1)
    for i, n := range arr {
        xorarr[i+1] = xorarr[i] ^ n
    }
    ans := 0
    for i:=1; i < len(arr); i++ {
        for j:=i+1; j < len(arr) + 1; j++ {
            if xorarr[i-1] == xorarr[j] {
                ans += (j-i)
            }
        }
    }
    return ans
}

