// LEARN

import "strconv"

func countAndSay(n int) string {
    if n == 1 {
        return "1"
    }
    
    s := countAndSay(n-1) 
    ans := ""
    
    curr := s[0]
    count:= 0
    for i:= 0; i< len(s); i++ {
        if s[i] == curr {
            count++
        } else {
            if count > 0 {
                ans += strconv.Itoa(count) + string(curr)
                count = 1
                curr = s[i]
            }
        }
    }
    if count > 0 {
        ans += strconv.Itoa(count) + string(curr)
        count = 0
        curr = byte(0)
    }
    return ans
}

/* */

// LEARN

func countAndSay(n int) string {
    b := []byte{'1'}
    for n > 1 {
        b1 := []byte{'0', b[0]}
        for _, c := range b {
            if c == b1[len(b1)-1] {
                b1[len(b1)-2]++
                continue
            }
            b1 = append(b1, '1')
            b1 = append(b1, c)
        }
        b = b1
        n--
    }
    return string(b)
}


