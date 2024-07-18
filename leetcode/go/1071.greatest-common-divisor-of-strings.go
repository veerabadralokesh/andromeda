func gcdOfStrings(str1 string, str2 string) string {
    gcd_len := gcd(len(str1), len(str2))
    common_substring := str1[:gcd_len]
    for i:=gcd_len; i < len(str1); i+=gcd_len {
        if str1[i:i+gcd_len] != common_substring {
            return ""
        }
    }
    for i:=0; i < len(str2); i+=gcd_len {
        if str2[i:i+gcd_len] != common_substring {
            return ""
        }
    }
    return common_substring
}

func gcd(a int, b int) int {
    if b == 0 {return a}
    return gcd(b, a % b)
}


