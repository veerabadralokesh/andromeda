func numTeams(rating []int) int {
    ans, ll, lm, rl, rm := 0, 0, 0, 0, 0
    for i:=1; i < len(rating); i++ {
        ll, lm, rl, rm = 0, 0, 0, 0
        for j:=0; j < i; j++ {
            if rating[i] < rating[j] {
                lm++
            } else {
                ll++
            }
        }
        for j:=i+1; j < len(rating); j++ {
            if rating[i] < rating[j] {
                rm++
            } else {
                rl++
            }
        }
        ans += (lm * rl + ll * rm)
    }
    return ans
}

