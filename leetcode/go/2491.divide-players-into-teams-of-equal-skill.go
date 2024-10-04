func dividePlayers(skill []int) int64 {
    n := len(skill)/2
    total := 0
    for _, s := range skill {
        total += s
    }
    if total % n != 0 {
        return -1
    }
    teamSkill := total / n
    rem := make([]int, 1 + teamSkill)
    for i := range teamSkill {
        rem[i] = 0
    }
    for _, s := range skill {
        if s >= teamSkill {
            return -1
        }
        rem[s]++
    }
    var ans int64 = 0
    for i:=1; i < 1 + (teamSkill/2); i++ {
        if rem[i] != rem[teamSkill-i] {
            return -1
        }
        if i != teamSkill - i {
            ans += int64(i) * int64(teamSkill - i) * int64(rem[i])
        } else {
            if rem[i] % 2 != 0 {return -1}
            ans += int64(i) * int64(teamSkill - i) * int64(rem[i])/2
        }
    }
    return ans
}

