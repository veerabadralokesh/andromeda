func numberOfUniqueGoodSubsequences(binary string) int {
    dp0, dp1 := 0, 0
    zero := 0
    for i:=0; i < len(binary); i++ {
        if binary[i] == '0' {
            dp0 = (dp0 + dp1) % 1_000_000_007
            zero = 1
        } else {
            dp1 = (dp1 + dp0 + 1) % 1_000_000_007
        }
    }
    return (dp0 + dp1 + zero) % 1_000_000_007
}

