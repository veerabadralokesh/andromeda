func xorQueries(arr []int, queries [][]int) []int {
    xors := make([]int, len(arr) + 1)
    for i := range len(arr) {
        xors[i+1] = xors[i] ^ arr[i]
    }
    ans := make([]int, len(queries))
    for i, q := range queries {
        ans[i] = xors[q[1] + 1] ^ xors[q[0]]
    }
    return ans
}

