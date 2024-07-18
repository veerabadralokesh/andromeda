func repeatedSubstringPattern(s string) bool {
    l := len(s)
    for i:=1; i < l/2+1 ; i++ {
        if l % i == 0 {
            flag := true
            for j := i; j < l; j++ {
                if s[j] != s[j-i] {
                    flag = false
                    break
                }
            }
            if flag {
                return true
            }
        }
    }
    return false
}

