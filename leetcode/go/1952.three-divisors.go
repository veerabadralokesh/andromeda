func isThree(n int) bool {
    count := 0
    for i:=1; i < n+1; i++ {
        if n % i == 0 {
            count++
        }
    }
    return count == 3
}

