import (
    "strings"
)

var tens = [10]string {
    "", "Ten", "Twenty", "Thirty", "Forty", "Fifty", 
    "Sixty", "Seventy", "Eighty", "Ninety",
}

var numbers = [20]string {
    "", "One", "Two", "Three",
    "Four", "Five", "Six", "Seven",
    "Eight", "Nine", "Ten", "Eleven",
    "Twelve", "Thirteen", "Fourteen", "Fifteen",
    "Sixteen", "Seventeen", "Eighteen", "Nineteen",
}

func helper(num int) string {
    s := ""
    if num < 20 {
        s = numbers[num]
    } else if num < 100 {
        s = tens[num/10] + " " + numbers[num % 10]
    } else if num < 1000 {
        s = numbers[num/100] + " Hundred " + helper(num % 100)
    } else if num < 1000000 {
        s = helper(num/1000) + " Thousand " + helper(num % 1000)
    } else if num < 1000000000 {
        s = helper(num/1000000) + " Million " + helper(num % 1000000)
    } else {
        s = helper(num/1000000000) + " Billion " + helper(num % 1000000000)
    }
    s = strings.TrimSpace(s)
    return s
}

func numberToWords(num int) string {
    if num == 0 {
        return "Zero"
    }
    return helper(num)
}


