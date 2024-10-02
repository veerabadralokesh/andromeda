func arrayRankTransform(arr []int) []int {
    if len(arr) == 0 {
        return arr
    }
    orig := make([]int, len(arr))

    for i, n := range arr {
        orig[i] = n
    }

    sort.Ints(arr)

    ranks := make(map[int]int)
    ranks[arr[0]] = 1
    rank := 2

    for i := 1; i < len(arr); i++ {
        if arr[i] != arr[i-1] {
            ranks[arr[i]] = rank
            rank++
        }
    }

    ans := make([]int, len(arr))
    for i, n := range orig {
        ans[i] = ranks[n]
    }
    return ans
}

