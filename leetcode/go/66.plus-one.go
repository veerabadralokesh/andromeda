func plusOne(digits []int) []int {
    ans := make([]int, len(digits) + 1)
    carry := 1
    for i:=len(digits); i > 0 ; i-- {
        ans[i] = digits[i-1] + carry
        if ans[i] > 9 {
            ans[i]-= 10
            carry = 1
        } else {
            carry = 0
        }
    }
    if carry == 0 {
        return ans[1:]
    } else {
        ans[0] = 1
        return ans
    }
}

/* */
func plusOne(digits []int) []int {
    n := len(digits)

    for i:=n-1; i>=0; i-- {
        if digits[i] < 9 {
            digits[i] = digits[i] + 1
            return digits
        } else {
            digits[i] = 0
        } 
    }

    if digits[0] == 0 {
        digits = append([]int{1},digits...)
    }

    return digits

}

