func lexicalOrder(n int) []int {
    ans := make([]int, 0, n+1)
    for i:=1; i < 10 && i <= n; i++ {
        genNums(i, n, &ans)
    }
    return ans
}

func genNums(i int, n int, ans *[]int) {
    (*ans) = append(*ans, i)
    for j :=i * 10; j < i * 10 + 10; j++ {
        if j > n {
            break
        }
        genNums(j, n, ans)
    }
}

