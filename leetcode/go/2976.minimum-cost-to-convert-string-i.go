func minimumCost(source string, target string, original []byte, changed []byte, cost []int) int64 {
    or := []byte(original)
    ch := []byte(changed)
    cc := [26][26]int64{}
    for i := range 26 {
        for j := range 26 {
            cc[i][j] = math.MaxInt64
        }
    }
    for i, o := range or {
        cc[o-97][ch[i]-97] = min(cc[o-97][ch[i]-97], int64(cost[i]))
    }
    for k := range 26 {
        for i := range 26 {
            if cc[i][k] < math.MaxInt64 {
                for j := range 26 {
                    if cc[k][j] < math.MaxInt64 {
                        cc[i][j] = min(cc[i][j], cc[i][k] + cc[k][j])
                    }
                }
            }
        }
    }
    sb := []byte(source)
    tb := []byte(target)
    var ans int64 = 0
    for i, s := range sb {
        if s != tb[i] {
            if cc[s-97][tb[i]-97] == math.MaxInt64 {
                return -1
            }
            ans += cc[s-97][tb[i]-97]
        }
    }
    return ans
}

func min(a, b int64) int64 {
    if a < b {
        return a
    }
    return b
}

